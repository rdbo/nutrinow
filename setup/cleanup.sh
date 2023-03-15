#!/bin/sh

psql -U postgres -f "$HOME/nutrinow/setup/delete_expired_sessions.sql"
