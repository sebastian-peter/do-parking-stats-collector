CREATE TABLE parking_stats (
   id INTEGER NOT NULL,
   source_time TIMESTAMP NOT NULL,
   opened BOOLEAN NOT NULL,
   updated_time TIMESTAMP DEFAULT NOW() NOT NULL,
   current_total_capacity SMALLINT NOT NULL,
   current_short_capacity SMALLINT NOT NULL,
   current_other_capacity SMALLINT NOT NULL,
   current_total_occupied SMALLINT NOT NULL,
   current_short_occupied SMALLINT NOT NULL,
   current_other_occupied SMALLINT NOT NULL,
   PRIMARY KEY (id, source_time),
   FOREIGN KEY (id) REFERENCES parking_info(id)
)
