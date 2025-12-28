#!/bin/bash

ROUTER_IP="192.168.8.1"

# Try one ping with 2 second timeout
if ! ping -c 1 -W 2 "$ROUTER_IP" > /dev/null ; then
    echo "$(date): Lost connection. Restarting Wi-Fi..." >> /var/log/wifi-recover.log

    # Bring Wi-Fi down and up using NetworkManager
    nmcli radio wifi off
    sleep 5
    nmcli radio wifi on
fi
