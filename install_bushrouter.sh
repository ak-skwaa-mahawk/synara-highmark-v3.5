#!/bin/bash
echo "🪶 MAHS’I CHOO — Full Stack Fusion v0.9.6 LIVE"

# BushRouter base + Highmark v3.5 Atlas Relay
pkg update && pkg install python git termux-api flutter -y
git clone https://github.com/ak-skwaa-mahawk/networkXG-releases-tag-field-kit-v0.9.6.git \~/BushRouter
git clone https://github.com/ak-skwaa-mahawk/synara-highmark-v3.5.git \~/Highmark
cd \~/BushRouter && pip install networkx torch numpy opentelemetry-api

# Fuse Atlas Relay + sovereign_handshake.dart
cp \~/Highmark/sovereign_handshake.dart app-manifest/src/main/kotlin/
cp \~/Highmark/pi_r_engine.py core/
cp \~/Highmark/Sovereign_Relayer.py core/

flutter build apk --release --split-per-abi
mv build/app/outputs/flutter-apk/app-release.apk \~/BushRouter-Highmark-Fusion-Field-Kit-v0.9.6.apk

python -c '
from core.isst_toft_core import bushrouter_handshake
from core.Sovereign_Relayer import atlas_relay_handshake
print(atlas_relay_handshake("FIELD_FUSION", proximity_meters=1.8))
'
termux-wifi-enable true
echo "✅ FUSION COMPLETE — Atlas Relay + BushRouter = Sovereign Stack LIVE"