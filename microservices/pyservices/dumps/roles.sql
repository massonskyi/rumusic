--
-- PostgreSQL database cluster dump
--

SET default_transaction_read_only = off;

SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;

--
-- Roles
--

CREATE ROLE postgres;
ALTER ROLE postgres WITH SUPERUSER INHERIT CREATEROLE CREATEDB LOGIN REPLICATION BYPASSRLS PASSWORD 'SCRAM-SHA-256$4096:xhwANaG7ZeqZ0feg8/j7Yw==$FHzCjb2+P5k7437P0pGUOg//aX0eO0Qwf+lRSzGmYSU=:U+pQ5x4r1VzqAmjhF+bhYI9wP+hSJ/JK7QQi+UaDJGQ=';

--
-- User Configurations
--






--
-- PostgreSQL database cluster dump complete
--

