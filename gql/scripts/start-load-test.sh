#!/bin/bash
vegeta attack -targets=vegeta-target -duration=600s -keepalive -name 'API Stress Test' -max-connections=700 -max-workers=200 -rate=250/5ms -workers=100 -timeout=1s | tee results.bin | vegeta report
