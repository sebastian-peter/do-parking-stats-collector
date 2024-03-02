#!/bin/sh

# setup once
dpsc --setup

# then run cron job
supercronic /cron/stats_collector
