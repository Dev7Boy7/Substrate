
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

error: failed to select a version for `parity-util-mem`.
    ... required by package `sp-core v5.0.0 (https://github.com/paritytech/substrate.git?tag=devhub/latest#22d40c76)`
    ... which satisfies git dependency `sp-core` of package `frame-support v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=devhub/latest#22d40c76)`
    ... which satisfies git dependency `frame-support` of package `frame-system v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=devhub/latest#22d40c76)`
    ... which satisfies git dependency `frame-system` of package `pallet-node-authorization v4.0.0-dev (https://github.com/paritytech/substrate.git?tag=devhub/latest#22d40c76)`
    ... which satisfies git dependency `pallet-node-authorization` of package `node-template-runtime v4.0.0-dev (/mnt/c/Users/trinc/Substrate-tutorial/substrate-node-template/runtime)`
    ... which satisfies path dependency `node-template-runtime` (locked to 4.0.0-dev) of package `node-template v4.0.0-dev (/mnt/c/Users/trinc/Substrate-tutorial/substrate-node-template/node)`
versions that meet the requirements `^0.10.2` are: 0.10.2

the package `parity-util-mem` links to the native library `parity-util-mem-ban-duplicates`, but it conflicts with a previous package which links to `parity-util-mem-ban-duplicates` as well:
package `parity-util-mem v0.11.0`
    ... which satisfies dependency `parity-util-mem = "^0.11"` (locked to 0.11.0) of package `kvdb v0.11.0`
    ... which satisfies dependency `kvdb = "^0.11.0"` (locked to 0.11.0) of package `frame-benchmarking-cli v4.0.0-dev (https://github.com/paritytech/substrate.git?branch=polkadot-v0.9.23#6cbe1772)`
    ... which satisfies git dependency `frame-benchmarking-cli` (locked to 4.0.0-dev) of package `node-template v4.0.0-dev (/mnt/c/Users/trinc/Substrate-tutorial/substrate-node-template/node)`
Only one package in the dependency graph may specify the same links value. This helps ensure that only one copy of a native library is linked in the final binary. Try to adjust your dependencies so that only one package uses the links ='parity-util-mem' value. For more information, see https://doc.rust-lang.org/cargo/reference/resolver.html#links.

failed to select a version for `parity-util-mem` which could resolve this conflict
    
    
*/
