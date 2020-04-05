create table "order"
(
    id integer not null
        constraint order_pk
            primary key autoincrement,
    item integer not null,
    amount integer not null,
    total float not null
);

create unique index order_id_uindex
    on "order" (id);

create index order_item_index
    on "order" (item);