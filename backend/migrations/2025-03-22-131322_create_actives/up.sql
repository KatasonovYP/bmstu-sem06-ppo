CREATE TABLE actives (
    active_id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    security_id INTEGER NOT NULL,
    bought_price INTEGER NOT NULL,
    count INTEGER NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(tg_id) ON DELETE CASCADE
);

CREATE INDEX idx_actives_user_id ON actives(user_id);
