/*  Simulate a network 

    - Hướng dẫn này giới thiệu cho bạn khởi đầu một blockchain riêng với các thẩm quyền riêng 

    - mô phỏng 2 node chạy trên cùng một máy tính nhằm mcuj đích xem các làm việc của mô hình đồng thuận 

    - Mục tiêu :

        1. Bắt đầu 1 blockchain với tài khoản định trước 

        2. Học các dòng lệnh quan trọng

        3. Xác định node có đang chạy tạo khối không 

        4. Kết nối với nút thứ 2 với mạng đang chạy

        5. Xác minh các máy tính ngang hàng đang sản xuất khối 


    - Bắt đầu với node đầu tiên ( 
        
    - Làm theo hướng dẫn chi tiết https://docs.substrate.io/tutorials/get-started/simulate-network/

        1. Chạy node 1 

        2. Chạy node 2

       
    - Một số lưu ý về command-line 

        1. --base-path : Chỉ định thư mục lưu trữ dữ liêu liên quan đến chuỗi

        2. --chain local : chỉ định đặc điểm sử dụng của chuỗi 

        3. --port 30333 của Alice và --port 30334 của Bob : Do ví dụ này chạy trên cùng 1 máy tính nên cần chỉ rõ cho 2 cái khác nhau . Tương tự --ws-port --rpc-port

        4. --node-key <key> : Chỉ định khóa bí mật

        5. --validator 

*/
