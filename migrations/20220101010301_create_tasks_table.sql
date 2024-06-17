CREATE TABLE tasks (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    project_id UUID REFERENCES projects(id)
);
