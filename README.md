# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
1. Melihat dari sisi kebutuhan saat ini, belum sepenuhnya diperlukan trait karena semua subscriber adalah Rocket instance yang menerima notifikasi melalui HTTP POST dengan perilaku yang sama. Dengan kata lain, hanya ada satu "tipe" subscriber, yaitu melalui endpoint HTTP. 

Namun, menggunakan trait akan memberikan fleksibilitas lebih untuk pengembangan di masa depan, misalnya jika ingin menambahkan mekanisme notifikasi lain seperti email atau WebSocket. Dengan trait, kita bisa memiliki implementasi yang berbeda tanpa harus mengubah kode publisher, sesuai dengan prinsip desain Observer yang memisahkan publisher dari concrete observers.

2. Dalam kasus ini, menggunakan Vec akan membuat operasi pencarian, penambahan, dan penghapusan menjadi tidak efisien karena harus melakukan iterasi untuk menemukan subscriber yang sesuai. Vec mencari key secara linear sehingga memiliki kompleksitas O(n), sedangkan DashMap menggunakan hash map yang memungkinkan pencarian, penambahan, dan penghapusan dalam waktu rata-rata O(1). Selain itu, DashMap sudah thread-safe, sehingga lebih aman digunakan dalam konteks aplikasi web yang mungkin memiliki banyak thread. DashMap juga secara otomatis memastikan bahwa setiap key (url) unik, sehingga tidak perlu melakukan pengecekan manual untuk mencegah duplikasi. Oleh karena itu, DashMap lebih cocok untuk kasus ini dibandingkan dengan Vec.


3. Dua hal ini menyelesaikan masalah yang berbeda, yaitu Singleton memastikan hanya ada satu instance global, sedangkan DashMap menyediakan state bersama yang aman untuk thread. Kita membutuhkan keduanya: Singleton (melalui lazy_static!) untuk memastikan hanya ada satu koleksi SUBSCRIBERS, dan DashMap untuk memungkinkan akses read/write yang aman secara bersamaan dari banyak request handlers (threads), sehingga kita tidak perlu membuat thread-safe wrapper sendiri untuk koleksi tersebut (misalnya, menggunakan Mutex<HashMap<..>>). Hal ini penting agar aplikasi web kita dapat menangani banyak permintaan secara bersamaan dengan efisien dan tanpa masalah race condition.

#### Reflection Publisher-2
1. Pemisahan ini merepresentasikan Single Responsibility Principle (SRP) dari prinsip SOLID dan konsep Hexagonal Architecture. Dalam MVC klasik, Model sering kali menjadi Fat Model karena menangani representasi data, aturan bisnis, dan interaksi database sekaligus. Model seharusnya murni berfokus pada struktur data dan logika bisnis inti yang spesifik untuk entitas tersebut. Model tidak boleh tahu menahu tentang database (SQL, NoSQL, dll). Repository bertanggung jawab penuh atas persistensi data. Layer ini mengabstraksi operasi database (CRUD). Jika suatu saat aplikasi bermigrasi dari PostgreSQL ke MongoDB, perubahan hanya terjadi di layer Repository tanpa menyentuh Model atau Service. Service bertindak sebagai orkestrator yang menangani business use case. Layer ini memanggil Repository untuk mengambil data (Model), menerapkan logika bisnis atau algoritma yang melibatkan beberapa Model sekaligus, lalu menyimpan kembali hasilnya melalui Repository. Pemisahan ini membuat kode menjadi loosely coupled, mudah di-maintain, dan sangat mudah untuk dilakukan Unit Testing (karena Repository dapat di-mock saat menguji Service).

2. Jika kita memaksakan semua logika hanya dalam Model, kita akan menciptakan Tight Coupling (ketergantungan yang ketat) dan kompleksitas kode. Ketika sebuah `Program` mengubah status dan harus mengirimkan `Notification` untuk semua `Subscriber`, maka logika untuk mengelola daftar subscriber, membuat payload notifikasi, dan mengirimkannya harus berada di dalam Model `Program`. Hal ini akan membuat Model `Program` menjadi sangat besar (God Object) dan sulit untuk dipahami, diuji, atau di-maintain. Model `Program` kini memiliki dependensi langsung pada Model `Subscriber` dan `Notification`, sehingga setiap perubahan pada salah satu model dapat mempengaruhi yang lain, meningkatkan risiko bug dan membuat refactoring menjadi sulit. Selain itu, jika kita ingin menambahkan jenis notifikasi baru atau mekanisme pengiriman baru, kita harus mengubah Model `Program`, yang seharusnya tidak perlu tahu tentang detail implementasi notifikasi.

3. Saya sudah cukup familiar dengan Postman, terutama untuk menguji REST API. Postman memudahkan saya untuk membuat dan mengelola koleksi endpoint, menyimpan environment variables, dan melakukan testing secara manual maupun otomatis. Fitur yang saya sukai adalah kemampuan untuk menyimpan response history, sehingga saya bisa dengan mudah melihat kembali hasil request sebelumnya. Fitur environment variables juga sangat membantu untuk mengelola konfigurasi yang berbeda (misalnya, development vs production) tanpa harus mengubah kode atau request secara manual. Saya yakin fitur-fitur ini akan sangat berguna untuk Group Project dan proyek software engineering lainnya di masa depan. Saya juga sangat suka compatibility Postman dengan cURL, sehingga saya bisa dengan mudah mengkonversi request yang saya buat di Postman ke dalam bentuk cURL command untuk digunakan di terminal atau script (ataupun sebaliknya). 

#### Reflection Publisher-3
