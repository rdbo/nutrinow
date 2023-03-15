DELETE FROM user_session WHERE expiry_date < NOW();
