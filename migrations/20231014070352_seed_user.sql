-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
        'ab2a9d2f-ed83-4be0-a39a-7dbd15bda863',
        'admin',
        '$argon2id$v=19$m=15000,t=2,p=1$wqAi3b6WKmFxRPQCxWvwtA$q0XsUd6rMnr72ruJusEmb3cdfS2kSj0HjkcfKfUz6Qs'
       );