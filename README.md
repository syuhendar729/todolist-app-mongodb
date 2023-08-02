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
```

### Berikut adalah API yang dapat digunakan:

| Method   | URL | 
| -------- | -------- | 
| GET      | `http://localhost:8080/todolist`   | 
| POST     | `http://localhost:8080/todolist`   |
| PUT      | `http://localhost:8080/todolist/{_id}`   |
| DELETE   | `http://localhost:8080/todolist/{_id}`   |

Created by [syuhendar](https://callmedar.github.io)
