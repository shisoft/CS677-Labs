create table item
(
    id integer not null
        constraint item_pk
            primary key autoincrement,
    name varchar not null,
    stock integer not null,
    price float not null,
    topic integer not null
);

create unique index item_id_uindex
    on item (id);

create index item_topic_index
    on item (topic);

