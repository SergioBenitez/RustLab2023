CREATE TABLE users (
    id          INTEGER     PRIMARY KEY AUTOINCREMENT,
    name        VARCHAR     NOT NULL,
    email       VARCHAR     NOT NULL,
    password    VARCHAR     NOT NULL,
    -- 0 = admin, 1 = doctor, 2 = patient
    role        INTEGER     NOT NULL DEFAULT 2 CHECK(role IN (0, 1, 2))
);
