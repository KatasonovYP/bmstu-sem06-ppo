CREATE TABLE notifications (
    notification_id SERIAL PRIMARY KEY,
    portfolio_id INTEGER NOT NULL,
    active_id INTEGER NOT NULL,
    limit_upper INTEGER NOT NULL,
    limit_lower INTEGER NOT NULL,
    limit_type SMALLINT NOT NULL,
    CONSTRAINT fk_active FOREIGN KEY (active_id) REFERENCES actives(active_id) ON DELETE CASCADE,
    CONSTRAINT fk_portfolio FOREIGN KEY (portfolio_id) REFERENCES users(user_id) ON DELETE CASCADE
);

CREATE INDEX idx_notifications_active_id ON notifications(active_id);

CREATE INDEX idx_notifications_portfolio_id ON notifications(portfolio_id);
