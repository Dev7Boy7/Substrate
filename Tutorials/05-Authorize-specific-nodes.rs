
/*   
  Authorize specific nodes
  
    Trong phần trước , đã học cách thiết lập một mạng blockchain đơn giản được cấp phép . 
    
    Chỉ có các node được ủy quyền - authorized nodes mới được thực hiện các hoạt động chuyên biệt 
    
    Các node còn lại tuyên truyền các giao dịch ? ( propagate transactions ) 
    
    Mạng cấp phép khác với mạng không cấp phép ( permissions - permissionless ) 
    
    Mạng không cấp phép thì trao quyền lớn hơn cho mạng lưới 
    
    Mạng cấp phép sẽ phù hợp với mạng lưới doanh nghiệp , tiền tệ quốc gia , sức khỏe ,.... 
  
  
  Node authorization and ownership
  
    Pallet node - authorization : được xây dựng sẵn cho phép bạn quản lý cấu hình 1 tập hợp các node trong mạng lưới .
    
    Mỗi node có PeerId riêng sở hữu bởi 1 AccountId xác nhận nút.
    
    Có hai cách ủy quyền cho node tham gia vào mạng lưới.
    
      1. Thêm PeerId vào danh sách node xác định trước . Cần sự chấp thuận của ban quản trị hoặc Sudo pallet 
      
      2. Yêu cầu kết nối ngang hàng với 1 node cụ thể , có thể có PeerId xác định hoặc node bình thường 
    
    Offchain worker - cần bật khi khởi đông node thì nó mặc định tắt với các node không có thẩm quyền 
    
  
  Mục tiêu :
  
    Kiểm tra và biên dịch các Node-template
    
    Thêm pallet được ủy quyền vào node đang chạy
    
    Khởi chạy nhiều node và cho phép các node mới tham gia
    
    
  Build the node template (https://docs.substrate.io/tutorials/get-started/permissioned-network/)
  
  
  Add the node authorization pallet
    
    Trong Substrate , file Cargo.toml kiểm soát 2 phần thông tin quan trọng 
        
        1. Các Pallet được imported vào - vị trí và phiên bản 
        
        2. The features in each pallet that should be enabled when compiling the native Rust binary. 
        By enabling the standard (std) feature set from each pallet, you can compile the runtime to include functions, 
        types, and primitives that would otherwise be missing when you build the WebAssembly binary.
    
    
   Add note-authorization dependencies 
   
    Theo hướng dẫn
   (https://docs.substrate.io/tutorials/get-started/permissioned-network/)
   
   * sau khi thêm 
   
   [dependencies]
pallet-node-authorization = { default-features = false, git = "https://github.com/paritytech/substrate.git", tag = "devhub/latest", version = "4.0.0-dev" }
    
    [features]
default = ['std']
std = [
 ...
 "pallet-node-authorization/std",    # add this line
 ...
]
    
=> chạy cargo check -p node-template-runtime => LỖI ............ 
    
    
*/
