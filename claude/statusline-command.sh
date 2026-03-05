#!/usr/bin/env bash
# Claude Code statusline — 3-row format with rate limit progress bars
# Row 1: directory on branch [status] via model [context%]
# Row 2: 5-hour rate limit bar
# Row 3: 7-day  rate limit bar

# ── Constants ────────────────────────────────────────────────────────────────
CACHE_FILE="/tmp/claude-usage-cache.json"
CACHE_TTL=360   # seconds (6 minutes)
TZ_DISPLAY="Asia/Seoul"

# 24-bit hex colors
CLR_GREEN="#97C9C3"
CLR_YELLOW="#E5C07B"
CLR_RED="#E06C75"
CLR_CYAN="#56B6C2"
CLR_MAGENTA="#C678DD"
CLR_BLUE="#61AFEF"

# ── ANSI helpers ─────────────────────────────────────────────────────────────
# c <hex> <text>  — wrap text in 24-bit foreground color
c() {
    local hex="${1#\#}"
    local r=$((16#${hex:0:2}))
    local g=$((16#${hex:2:2}))
    local b=$((16#${hex:4:2}))
    printf "\033[38;2;%d;%d;%dm%s\033[0m" "$r" "$g" "$b" "$2"
}

# ── Color by percentage (0-100 int) ──────────────────────────────────────────
pct_color() {
    local pct=$1
    if   (( pct < 50 )); then echo "$CLR_GREEN"
    elif (( pct < 80 )); then echo "$CLR_YELLOW"
    else                      echo "$CLR_RED"
    fi
}

# ── Progress bar (10 segments) ───────────────────────────────────────────────
make_bar() {
    local pct=$1
    local filled=$(( (pct * 10 + 50) / 100 ))   # round
    (( filled > 10 )) && filled=10
    local empty=$(( 10 - filled ))
    local bar="" i
    for (( i=0; i<filled; i++ )); do bar+="▰"; done
    for (( i=0; i<empty;  i++ )); do bar+="▱"; done
    echo "$bar"
}

# ── Time formatting (gdate preferred, date fallback) ─────────────────────────
# format_time_short <iso8601>  →  "H:MM"  (e.g. "3:00")
format_time_short() {
    TZ="$TZ_DISPLAY" gdate -d "$1" +"%-H:%M" 2>/dev/null || echo "?"
}

# format_time_long <iso8601>  →  "MM-DD H:MM"  (e.g. "03-06 3:00")
format_time_long() {
    TZ="$TZ_DISPLAY" gdate -d "$1" +"%m-%d %-H:%M" 2>/dev/null || echo "?"
}

# ── float → integer percentage (API returns 0-100 float) ────────────────────
to_int_pct() {
    local val="$1"
    [[ -z "$val" || "$val" == "null" ]] && return
    printf "%.0f" "$val" 2>/dev/null
}

# ── Parse stdin (single jq call, IFS=$'\t') ──────────────────────────────────
input=$(cat)

IFS=$'\t' read -r cwd model_display used_pct git_branch_json < <(
    echo "$input" | jq -r '[
        .workspace.current_dir,
        .model.display_name,
        (.context_window.used_percentage // ""),
        (.git.branch             // "")
    ] | @tsv'
)

# Fallback: get branch from git if not in JSON
if [[ -z "$git_branch_json" ]]; then
    git_branch_json=$(git --no-optional-locks rev-parse --abbrev-ref HEAD 2>/dev/null || echo "")
fi

home_replaced="${cwd/#$HOME/\~}"

# ── Fetch / cache rate-limit data ─────────────────────────────────────────────
fetch_usage() {
    local raw_creds
    raw_creds=$(security find-generic-password -s "Claude Code-credentials" -w 2>/dev/null) || return 1
    local token
    token=$(echo "$raw_creds" | jq -r '.claudeAiOauth.accessToken // empty' 2>/dev/null) || return 1
    [[ -z "$token" ]] && return 1

    curl -s --max-time 5 \
        -H "Authorization: Bearer $token" \
        -H "anthropic-beta: oauth-2025-04-20" \
        "https://api.anthropic.com/api/oauth/usage" 2>/dev/null
}

usage_json=""

# Use cache if fresh
if [[ -f "$CACHE_FILE" ]]; then
    cache_age=$(( $(date +%s) - $(stat -f %m "$CACHE_FILE" 2>/dev/null || echo 0) ))
    if (( cache_age < CACHE_TTL )); then
        usage_json=$(cat "$CACHE_FILE")
    fi
fi

# Fetch fresh if cache is stale / missing
if [[ -z "$usage_json" ]]; then
    fresh=$(fetch_usage)
    if [[ -n "$fresh" ]] && echo "$fresh" | jq -e . > /dev/null 2>&1; then
        usage_json="$fresh"
        echo "$fresh" > "$CACHE_FILE"
    elif [[ -f "$CACHE_FILE" ]]; then
        usage_json=$(cat "$CACHE_FILE")   # stale cache fallback
    fi
fi

# ── Extract rate-limit fields ────────────────────────────────────────────────
fh_pct_raw=""
fh_reset=""
sd_pct_raw=""
sd_reset=""

if [[ -n "$usage_json" ]]; then
    IFS=$'\t' read -r fh_pct_raw fh_reset sd_pct_raw sd_reset < <(
        echo "$usage_json" | jq -r '[
            (.five_hour.utilization  // ""),
            (.five_hour.resets_at    // ""),
            (.seven_day.utilization  // ""),
            (.seven_day.resets_at    // "")
        ] | @tsv' 2>/dev/null
    )
fi

fh_int=$(to_int_pct "$fh_pct_raw")
sd_int=$(to_int_pct "$sd_pct_raw")

# ── Row 1 ────────────────────────────────────────────────────────────────────
dir_part=$(c "$CLR_CYAN" "$home_replaced")

git_status_part=""
if [[ -n "$git_branch_json" ]]; then
    git_dirty=$(git --no-optional-locks -C "$cwd" status --porcelain 2>/dev/null)
    if [[ -z "$git_dirty" ]]; then
        status_indicator=$(c "$CLR_GREEN" "[✓]")
    else
        status_indicator=$(c "$CLR_RED" "[✗]")
    fi
    git_status_part=" $(c "$CLR_MAGENTA" "on") ${git_branch_json} ${status_indicator}"
fi

model_ctx_part=" $(c "$CLR_BLUE" "via") ${model_display}"
if [[ -n "$used_pct" ]]; then
    ctx_color=$(pct_color "$used_pct")
    model_ctx_part+=" $(c "$ctx_color" "[${used_pct}%]")"
fi

row1="${dir_part}${git_status_part}${model_ctx_part}"

# ── Row 2 (5-hour) ───────────────────────────────────────────────────────────
row2=""
if [[ -n "$fh_int" ]]; then
    bar=$(make_bar "$fh_int")
    clr=$(pct_color "$fh_int")
    reset_str=""
    [[ -n "$fh_reset" ]] && reset_str="  Resets $(format_time_short "$fh_reset")"
    row2="$(c "$clr" "5h")  $(c "$clr" "$bar")  $(c "$clr" "${fh_int}%")${reset_str}"
fi

# ── Row 3 (7-day) ────────────────────────────────────────────────────────────
row3=""
if [[ -n "$sd_int" ]]; then
    bar=$(make_bar "$sd_int")
    clr=$(pct_color "$sd_int")
    reset_str=""
    [[ -n "$sd_reset" ]] && reset_str="  Resets $(format_time_long "$sd_reset")"
    row3="$(c "$clr" "7d")  $(c "$clr" "$bar")  $(c "$clr" "${sd_int}%")${reset_str}"
fi

# ── Output ───────────────────────────────────────────────────────────────────
echo "$row1"
[[ -n "$row2" ]] && echo "$row2"
[[ -n "$row3" ]] && echo "$row3"
