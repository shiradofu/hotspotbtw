#!/usr/bin/env sh

cd "$(dirname "$0")/.." || exit 1
. .env

if [ -z "$APP_BIN_DIR" ]; then
  echo "APP_BIN_DIR is not set" 1>&2
  exit 1
fi

if [ -z "$APP_LOG_PARENT_DIR" ]; then
  echo "APP_LOG_PARENT_DIR is not set" 1>&2
  exit 1
fi
APP_LOG_DIR="$APP_LOG_PARENT_DIR/$APP_NAME"

# https://unix.stackexchange.com/a/598047
case "${APP_INTERVAL_SEC}" in
  (*[!0123456789]*)
    echo "APP_INTERVAL_SEC is invalid: $APP_INTERVAL_SEC" 1>&2
    exit 1
    ;;
  ('') APP_INTERVAL_SEC=300;;
  (*) ;;
esac

APP_NAME=hotspotbtw
plist_file="$HOME/Library/LaunchAgents/$APP_NAME.plist"

__deploy() {
  if ! mkdir -p "$APP_BIN_DIR"; then
    echo "failed to crate dir: $APP_BIN_DIR" 1>&2
    exit 1
  fi

  if ! cargo build --release ; then
    echo "cargo build failed" 1>&2
    exit 1
  fi

  cp -f "./target/release/$APP_NAME" "$APP_BIN_DIR/"

  launchctl unload "$plist_file" >/dev/null 2>&1
  rm -f "$plist_file"

  APP_NAME="$APP_NAME" \
  APP_BIN_DIR="$APP_BIN_DIR" \
  APP_LOG_DIR="$APP_LOG_DIR" \
  APP_INTERVAL_SEC=$APP_INTERVAL_SEC \
    envsubst < ./template.plist > "$plist_file" && \
  launchctl load "$plist_file" || {
    echo "failed to register the agent" 1>&2
    exit 1
  }
}

__undo() {
  launchctl unload "$plist_file"
  if [ -f "$plist_file" ]; then
    bin_path="$(plutil -extract ProgramArguments.0 raw -o - "$plist_file")"
    rm -f "$bin_path"
  fi
  rm -f "$plist_file"
}

for arg; do
  case $arg in
    --undo )
      __undo
      exit $?
      ;;
    * )
      echo "invalid arg: $arg" 1>&2
      exit 1
      ;;
  esac
done

__deploy
