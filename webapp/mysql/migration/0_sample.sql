-- このファイルに記述されたSQLコマンドが、マイグレーション時に実行されます。
alter table areas add index idx_areas_name(name);