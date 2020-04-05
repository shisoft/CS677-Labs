create table topic
(
    id integer not null
        constraint topic_pk
            primary key autoincrement,
    name varchar not null
);

create unique index topic_id_uindex
    on topic (id);