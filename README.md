# 🔗 rblink — A Simple Static File HTTP Server in Rust

`rblink` is a minimal HTTP server written in pure Rust that serves static files and directories over the `GET` method. It opens the browser automatically, displays a clean file listing, and runs without external dependencies.

---

## 🚀 Features

- 📁 Serves static files from the current directory
- 🧭 Clean directory listing in HTML
- 🖥️ Auto-opens the browser on launch
- 💡 No external crates (pure `std`)
- 🔒 GET-only HTTP support (status `405` on others)

---

## 🧪 Usage

```bash
cargo run <port>
```

Example:

```bash
cargo run 8080
```

This will:

* Bind to `127.0.0.1:8080`
* Serve the current folder contents
* Open your default browser pointing to `http://127.0.0.1:8080` (if supported)

---

## 📁 Directory Listing

If no `index.html` is found in the current directory, `rblink` generates a minimal HTML page listing all files and folders, with icons:

```html
📁 folder/
📄 file.txt
```

Clicking any item navigates or opens it directly.

---

## 🛑 Limitations

* No MIME-type detection (files are returned as text)
* No support for POST/PUT/DELETE
* Synchronous, single-threaded request handling
* No TLS (HTTP only)

---

## 🧠 Inspiration

This project was created for learning purposes and small local development use-cases (like previewing HTML folders).

---

## 📄 License

MIT License.