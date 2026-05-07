#!/bin/sh
# move_by_pid.sh
# $XDG_RUNTIME_DIR/haridokei.pid から PID を読み、wmctrl -lp で対応するウィンドウを探して右上に移動する (X11/wmctrl 必須)

PIDFILE="$XDG_RUNTIME_DIR/haridokei.pid"
MARGIN=16        # 画面端からの余白(px)
RETRIES=60       # 最大リトライ回数
SLEEP=0.1        # リトライ間隔(秒)

if [ ! -f "$PIDFILE" ]; then
  echo "pidfile not found: $PIDFILE" >&2
  exit 1
fi

APP_PID="$(cat "$PIDFILE" 2>/dev/null)"
if ! pgrep -x -P 1 -f "^$APP_PID$" >/dev/null 2>&1 && ! kill -0 "$APP_PID" 2>/dev/null; then
  # プロセス存在チェック（簡易）
  # 続ける場合はコメントアウトしても良い
  echo "process $APP_PID not running" >&2
  # exit 1
fi

# 画面サイズ（単純版: 仮想スクリーン全体）
SCREEN_W="$(xdpyinfo | awk '/dimensions:/ {print $2}' | awk -Fx '{print $1}')"
SCREEN_H="$(xdpyinfo | awk '/dimensions:/ {print $2}' | awk -Fx '{print $2}')"

i=0
while [ $i -lt $RETRIES ]; do
  WIN_ID="$(wmctrl -lp | awk -v pid="$APP_PID" '$3==pid { print $1; exit }')"
  if [ -n "$WIN_ID" ]; then
    # ウィンドウ x,y,w,h を取得
    read WIN_X WIN_Y WIN_W WIN_H <<EOF
$(wmctrl -lG | awk -v id="$WIN_ID" '$1==id {print $3, $4, $5, $6; exit }')
EOF
    # 右上に移動: x = SCREEN_W - WIN_W - MARGIN, y = MARGIN
    NEW_X=$((SCREEN_W - WIN_W - MARGIN))
    NEW_Y=$MARGIN
    wmctrl -ir "$WIN_ID" -e "0,$NEW_X,$NEW_Y,-1,-1"
    exit 0
  fi
  i=$((i+1))
  sleep "$SLEEP"
done

echo "window for pid $APP_PID not found after retries" >&2
exit 2
