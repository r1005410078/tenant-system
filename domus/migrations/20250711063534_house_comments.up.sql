
CREATE TABLE house_comments (
    id              VARCHAR(64) PRIMARY KEY,
    house_id        VARCHAR(64) NOT NULL,
    admin_id        VARCHAR(64) NOT NULL,
    content         TEXT NOT NULL,
    created_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_house_id (house_id),
    INDEX idx_admin_id (admin_id)
);