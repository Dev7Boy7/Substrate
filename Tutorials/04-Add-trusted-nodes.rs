/*  Add trusted nodes

    - Cách mà bạn có thể bắt đầu với mạng nhỏ , mạng blockchain độc lập với 1 bộ tập hợp riêng có thẩm quyền

    - Sự đồng thuận : là trạng thái tất cả các node trong mạng lưới blockchain đồng ý về trạng thái dữ liệu tại bất kì thời điểm nào 

    - Đồng thuận Aura : giới hạn sản xuất khối với danh sách luân phiên các tài khoản được ủy quyền . Authorities là các tài khoản được ủy quyền tạo khối theo kiểu vòng tròn 
    gọi là trusted participants trong mạng lưới.

    - Mục tiêu :

        1. Tạo các cặp khóa sử dụng cho quản lý mạng lưới

        2. Tạo  file mô tả đặc trưng cho chuỗi tùy chỉnh

        3. Khởi chạy 1 mạng blockchain 2 node riêng 
        

    - Các tùy chọn tạo khóa 

        + Sử dụng node-template : air-gapped computer , sử dụng máy tính cô lập vs mạng internet là tốt nhất hoặc ít nhất là tắt internet tại thời điểm tạo khóa 

            có hướng dẫn chi tiết tại https://docs.substrate.io/tutorials/get-started/trusted-network/

            VD
                Secret phrase:       family damp guess unveil athlete enact young index aunt ostrich grow chalk
                Network ID:        substrate
                Secret seed:       0x59bc3873e3454ef36fb2be3660aca3739a3132db539fc321a8b11a408cfdf54b
                Public key (hex):  0xe6b78c8b01a30a07ecb63a32ac9ca525c53b6562f43e5118d4200afc1dc97f10
                Account ID:        0xe6b78c8b01a30a07ecb63a32ac9ca525c53b6562f43e5118d4200afc1dc97f10
                Public key (SS58): 5HHDPvgHW6TX2YfZdGC61r5G3DAih9UhqeY8otUZXykm7PqB
                SS58 Address:      5HHDPvgHW6TX2YfZdGC61r5G3DAih9UhqeY8otUZXykm7PqB


    - Thay đổi trường aura để chỉ định các node được tạo khối bằng cách thêm các khóa địa chỉ Sr25519 SS58 ( Adress key )

    - Thay đổi trường grandpa để chỉ định các node được hoàn thiện khối bằng cách thêm các khóa địa chỉ Sr25519 SS58 ( Adress key ) . Lưu ý thêm chỉ số weighted votes 

    - Thêm Validator

    - Chuyển định dạng Raw cho file json -> chia sẻ cho những người tham gia trong mạng lưới ( tham khảo bài đọc https://dev.to/gnunicorn/hunting-down-a-non-determinism-bug-in-our-rust-wasm-build-4fk1)

    - Thực hành :

        Tạo 2 khóa :    
                1.
                Key password:
                Secret phrase:       duty light about vintage source hundred outdoor flee bitter among cause honey
                Network ID:        substrate
                Secret seed:       0x392f4204b20e5d8a414dcb67506042303e0473b991c936249b05c2822ff138aa
                Public key (hex):  0x0ab0394842272ddf1f1be1d64bb7890e94e7e3803632d6cce99ee40599d99a50
                Account ID:        0x0ab0394842272ddf1f1be1d64bb7890e94e7e3803632d6cce99ee40599d99a50
                Public key (SS58): 5CJighw7ZTbrQMGfTGJnSC7vvnhRkptxmNQuE7f5mo1XLMk7
                SS58 Address:      5CJighw7ZTbrQMGfTGJnSC7vvnhRkptxmNQuE7f5mo1XLMk7
                
                Secret phrase:       duty light about vintage source hundred outdoor flee bitter among cause honey
                Network ID:        substrate
                Secret seed:       0x392f4204b20e5d8a414dcb67506042303e0473b991c936249b05c2822ff138aa
                Public key (hex):  0x837fbf9306e326ca7421ade8fb6156738e2b95d45907a6cbb11f33085428ed34
                Account ID:        0x837fbf9306e326ca7421ade8fb6156738e2b95d45907a6cbb11f33085428ed34
                Public key (SS58): 5F385YpFNP8nSqRKCgnZcroEivk8iE1KVYAhLsfW6y7hzdDu
                SS58 Address:      5F385YpFNP8nSqRKCgnZcroEivk8iE1KVYAhLsfW6y7hzdDu


                2. 
                Key password:
                Secret phrase:       excite mosquito use improve little tired mail tiger maze saddle dragon carry
                Network ID:        substrate
                Secret seed:       0x1b621b1eb14dce927ebc003775abb778553ab22083cf8626c9f458aa4a854d5a
                Public key (hex):  0xfeb76b232311e5f10384c80242010ac2506491b700ebd370cbd73ecf10e1c761
                Account ID:        0xfeb76b232311e5f10384c80242010ac2506491b700ebd370cbd73ecf10e1c761
                Public key (SS58): 5HpgWLJ6mQSegGmySw7EpVwnoGcj2KA7m87BrWYhQXwdtVvt
                SS58 Address:      5HpgWLJ6mQSegGmySw7EpVwnoGcj2KA7m87BrWYhQXwdtVvt

                Key password:
                Secret phrase:       excite mosquito use improve little tired mail tiger maze saddle dragon carry
                Network ID:        substrate
                Secret seed:       0x1b621b1eb14dce927ebc003775abb778553ab22083cf8626c9f458aa4a854d5a
                Public key (hex):  0x40b11b99eb8e60cf8210b6084fcf157a711c9bfb6a26787f16bff4b7ad6ca9fb
                Account ID:        0x40b11b99eb8e60cf8210b6084fcf157a711c9bfb6a26787f16bff4b7ad6ca9fb
                Public key (SS58): 5DXXXuw2D3nSf7PZPDw3hmGZnK833GSDqeeUWKiwscMQZSwm
                SS58 Address:      5DXXXuw2D3nSf7PZPDw3hmGZnK833GSDqeeUWKiwscMQZSwm

                thu 1: 
                ./target/release/node-template key insert --base-path /mnt/c/Users/trinc/Substrate-tutorial/substrate-node-template/tmp/node01 \
                    --chain customSpecRaw.json \
                    --scheme Sr25519 \
                    --suri 0x392f4204b20e5d8a414dcb67506042303e0473b991c936249b05c2822ff138aa \
                    --password-interactive \
                    --key-type aura


                ./target/release/node-template key insert \
                    --base-path /mnt/c/Users/trinc/Substrate-tutorial/substrate-node-template/tmp/node01 \
                    --chain customSpecRaw.json \
                    --scheme Ed25519 \
                    --suri 0x392f4204b20e5d8a414dcb67506042303e0473b991c936249b05c2822ff138aa \
                    --password-interactive \
                    --key-type gran


                thu 2 : 
                ./target/release/node-template key insert --base-path /tmp/node02 \
                    --chain customSpecRaw.json \
                    --scheme Sr25519 \
                    --suri "excite mosquito use improve little tired mail tiger maze saddle dragon carry" \
                    --password-interactive \
                    --key-type aura


                ./target/release/node-template key insert --base-path /tmp/node02 \
                    --chain customSpecRaw.json \
                    --scheme Ed25519 --suri "excite mosquito use improve little tired mail tiger maze saddle dragon carry" \
                    --password-interactive \
                    --key-type gran


                ./target/release/node-template \
                    --base-path /tmp/node01 \
                    --chain ./customSpecRaw.json \
                    --port 30333 \
                    --ws-port 9945 \
                    --rpc-port 9933 \
                    --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
                    --validator \
                    --rpc-methods Unsafe \
                    --name MyNode01

                ./target/release/node-template \
                    --base-path /tmp/node02 \
                    --chain ./customSpecRaw.json \
                    --port 30334 \
                    --ws-port 9946 \
                    --rpc-port 9934 \
                    --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
                    --validator \
                    --rpc-methods Unsafe \
                    --name MyNode02 \
                    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWRR9b94WKSNCuoK6BH3r9EwyGns466gcUsKgKR5Zjkmmk \
                    --password-interactive


                Lỗi Bootnode with peer id 12D3KooWRR9b94WKSNCuoK6BH3r9EwyGns466gcUsKgKR5Zjkmmk is on a different chain (our genesis: 0xc088…a13a theirs: 0x7fe3…aab8) ;


                Thực hành lần 2:  
./target/release/node-template key generate --scheme Sr25519 --password-interactive

Secret phrase:       anger broom middle angry try treat void smooth ride fish fiscal metal
  Network ID:        substrate
  Secret seed:       0x2d324d8d546dfec810aee4787f03bf7163cf7d8965a2c1434ff2bb4b7ffac4c5
  Public key (hex):  0xce8c906782f573ce02ca8ebdfc8311cd29243e37c7c676e183b528085967cc32
  Account ID:        0xce8c906782f573ce02ca8ebdfc8311cd29243e37c7c676e183b528085967cc32
  Public key (SS58): 5GjXUf7TPa1zeciQAB51DHLo414CEukXKdvV2V4v2YTg1UVT
  SS58 Address:      5GjXUf7TPa1zeciQAB51DHLo414CEukXKdvV2V4v2YTg1UVT

./target/release/node-template key inspect --password-interactive --scheme Ed25519 "anger broom middle angry try treat void smooth ride fish fiscal metal" 

Secret phrase:       anger broom middle angry try treat void smooth ride fish fiscal metal
  Network ID:        substrate
  Secret seed:       0x2d324d8d546dfec810aee4787f03bf7163cf7d8965a2c1434ff2bb4b7ffac4c5
  Public key (hex):  0xec56405c2391c2e49785258ed5742a4f1bb6a5790c9d7ae170526c39762dbec3
  Account ID:        0xec56405c2391c2e49785258ed5742a4f1bb6a5790c9d7ae170526c39762dbec3
  Public key (SS58): 5HQanC5BYQSi6e2zbYtRbriLMjM75bqR9g1yzURpNcmSZ5av
  SS58 Address:      5HQanC5BYQSi6e2zbYtRbriLMjM75bqR9g1yzURpNcmSZ5av

Thu 2

Secret phrase:       oxygen decorate ready egg prefer anger fossil hard vocal flash topple clerk
  Network ID:        substrate
  Secret seed:       0x43b7c2229eaff25fde6fba946f7aca794ba279a480777bcedc952b57bec37857
  Public key (hex):  0xd8f1ecab4412e36e53bf8373d074fa32e5e6ec6410a0af4abdd65858b4db1334
  Account ID:        0xd8f1ecab4412e36e53bf8373d074fa32e5e6ec6410a0af4abdd65858b4db1334
  Public key (SS58): 5GyA4q78Ao1EYLUFp1dmgK1BD4RUUUxng2ZXEbL3s8QQEcjv
  SS58 Address:      5GyA4q78Ao1EYLUFp1dmgK1BD4RUUUxng2ZXEbL3s8QQEcjv


./target/release/node-template key inspect --password-interactive --scheme Ed25519 "oxygen decorate ready egg prefer anger fossil hard vocal flash topple clerk"

Secret phrase:       oxygen decorate ready egg prefer anger fossil hard vocal flash topple clerk
  Network ID:        substrate
  Secret seed:       0x43b7c2229eaff25fde6fba946f7aca794ba279a480777bcedc952b57bec37857
  Public key (hex):  0x4e68b420d24c318eeacbc664616ed0125f1d0a779f477349d39c9f54c2ebe29e
  Account ID:        0x4e68b420d24c318eeacbc664616ed0125f1d0a779f477349d39c9f54c2ebe29e
  Public key (SS58): 5DqWhJPa24PbxEWiZRyKaDZgeUkQkUCx9wNENtQzTuJGNPWW
  SS58 Address:      5DqWhJPa24PbxEWiZRyKaDZgeUkQkUCx9wNENtQzTuJGNPWW


----------------------------------

./target/release/node-template \
  --base-path /mnt/c/Users/trinc/Substrate-tutorial/substrate-node-template/tmp/node01 \
  --chain ./customSpecRaw.json \
  --port 30333 \
  --ws-port 9945 \
  --rpc-port 9933 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --validator \
  --rpc-methods Unsafe \
  --name MyNode01


12D3KooWGYmsdP687x9vwp3Fp1tEY1UHzprMANuxtCwZX7QwrLea

./target/release/node-template key insert --base-path /mnt/c/Users/trinc/Substrate-tutorial/substrate-node-template/tmp/node01 \
  --chain customSpecRaw.json \
  --scheme Sr25519 \
  --suri 0x2d324d8d546dfec810aee4787f03bf7163cf7d8965a2c1434ff2bb4b7ffac4c5 \
  --password-interactive \
  --key-type aura

./target/release/node-template key insert \
  --base-path /mnt/c/Users/trinc/Substrate-tutorial/substrate-node-template/tmp/node01 \
  --chain customSpecRaw.json \
  --scheme Ed25519 \
  --suri 0x2d324d8d546dfec810aee4787f03bf7163cf7d8965a2c1434ff2bb4b7ffac4c5 \
  --password-interactive \
  --key-type gran

ls /mnt/c/Users/trinc/Substrate-tutorial/substrate-node-template/tmp/node01/chains/local_testnet/keystore

./target/release/node-template \
  --base-path /mnt/c/Users/trinc/Substrate-tutorial/substrate-node-template/tmp/node02 \
  --chain ./customSpecRaw.json \
  --port 30334 \
  --ws-port 9946 \
  --rpc-port 9934 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --validator \
  --rpc-methods Unsafe \
  --name MyNode02 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWGYmsdP687x9vwp3Fp1tEY1UHzprMANuxtCwZX7QwrLea \
  --password-interactive



./target/release/node-template key insert --base-path /mnt/c/Users/trinc/Substrate-tutorial/substrate-node-template/tmp/node02 \
  --chain customSpecRaw.json \
  --scheme Sr25519 \
  --suri 0x43b7c2229eaff25fde6fba946f7aca794ba279a480777bcedc952b57bec37857 \
  --password-interactive \
  --key-type aura

./target/release/node-template key insert --base-path /mnt/c/Users/trinc/Substrate-tutorial/substrate-node-template/tmp/node02 \
  --chain customSpecRaw.json \
  --scheme Ed25519 --suri 0x43b7c2229eaff25fde6fba946f7aca794ba279a480777bcedc952b57bec37857 \
  --password-interactive \
  --key-type gran

  ls /mnt/c/Users/trinc/Substrate-tutorial/substrate-node-template/tmp/node02/chains/local_testnet/keystore




  DONEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE
  
*/
