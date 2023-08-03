# TodoList App
Dibuat dengan bahasa pemrograman [Rust](https://www.rust-lang.org/)  
Menggunakan [Actix Web](https://actix.rs/) sebagai framework  
Dan [MongoDB](https://www.mongodb.com/) sebagai database  
(tanpa front-end)

### Ingin mencoba? 

```bash
# Install Rust dan mongoDB dulu bagaimanapun caranya
# Jalankan mongoDB dan coba koneksikan ke mongoDB kompas terlebih dahulu
git clone <projek-ini>
cd <projek-ini>
cp env .env
# Tambah di file .env string koneksi database
cargo build
cargo run
# Gunakan Postman untuk testing APU
```

### Berikut adalah API yang dapat digunakan:

| Method   | URL                                      | Fungsi |
| -------- | ---------------------------------------  | ------ |
| GET      | `http://localhost:8080/todolist/{id}`    | Mengambil detail data berdasarkan ID |
| POST     | `http://localhost:8080/todolist`         | Menambah data |
| PUT      | `http://localhost:8080/todolist/{_id}`   | Mengedit data berdasarkan ID |
| DELETE   | `http://localhost:8080/todolist/{_id}`   | Menghapus data berdasarkan ID |


### Model Data 

Di MongoDB `_id: ObjectID()`:

```json
{
  "_id": {
    "$oid": "<id_defaut>"
  },
  "title": "Judul",
  "job": "Deskripsi Pekerjaan"
}
```

Menambah data `body > raw > json`:
```json
{
    "title": "Masukkan Judul",
    "job": "Masukkan deskripsi pekerjaan"
}
```

Memperbarui data `body > raw > json`:
```json
{
    "title": "Masukkan Judul Baru",
    "job": "Masukkan deskripsi pekerjaan baru"
}
```


Created by [syuhendar](https://callmedar.github.io)
