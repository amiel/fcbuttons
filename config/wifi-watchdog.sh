#!/bin/bash

ROUTER_IP="192.168.8.1"
REMOTE_RECOVER="/usr/local/bin/wifi-recover.sh"
STATEFILE="/var/run/wifi_failures.txt"
MAX_FAILURES=8
SLEEP_SECONDS=180

echo "wifi-watchdog started"

while true; do
    # If router reachable, reset failures
    if ping -c 1 -W 2 "$ROUTER_IP" > /dev/null; then
        echo 0 > "$STATEFILE"
    else
        # Increment failures
        FAILS=$(($(cat "$STATEFILE" 2>/dev/null || echo 0) + 1))
        echo "$FAILS" > "$STATEFILE"

        echo "$(date): Network unreachable ($FAILS failures)" >> /var/log/wifi-recovery.log

        # Try Wi-Fi recovery
        "$REMOTE_RECOVER"

        # If failures exceed limit, reboot
        if [ "$FAILS" -gt "$MAX_FAILURES" ]; then
            echo "$(date): Network recovery failed $FAILS times — rebooting..." >> /var/log/wifi-recovery.log
            reboot
        fi
    fi

    sleep "$SLEEP_SECONDS"
done
