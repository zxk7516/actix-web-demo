-- Your SQL goes here
CREATE TABLE "public"."links" (
  "id" serial4,
  "link" varchar(255) NOT NULL,
  "title" varchar(255) NOT NULL,
--  "date_created" varchar(255) NOT NULL,
  "date_created" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY ("id")
);
