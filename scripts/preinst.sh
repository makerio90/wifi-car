#!/usr/bin/env bash

systemctl-exists() {
  [ $(systemctl list-unit-files "${1}*" | wc -l) -gt 3 ]
}

systemctl-exists wifi-car && sudo systemctl stop wifi-car || true
