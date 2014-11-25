require 'openssl'

$key = File.read 'key'

def prep_cipher
  ci = OpenSSL::Cipher.new 'aes-256-cbc'
  ci.encrypt
  ci.key = $key
  { iv: ci.random_iv, enc: ci }
end

eheader = ""
eheader += "A:"               # File attrs

eheader += [2].pack 'N'       # Inode  [u64]
eheader += [1].pack 'N'       # Size   [u64]
eheader += [0].pack 'N'       # Blocks [u64]

eheader += [0,1,2].pack 'NNN' # Timespec (a) [i64 + i32]
eheader += [0,1,2].pack 'NNN' # Timespec (m)
eheader += [0,1,2].pack 'NNN' # Timespec (c)
eheader += [0,1,2].pack 'NNN' # Timespec (cr)

eheader += [0].pack 'x'       # FileType [u8]
eheader += [0644].pack('N')[2..3] # FilePermission [u32]

eheader += [0].pack('N')[2..3] # Nlink [u32]
eheader += [0].pack('N')[2..3] # Uid [u32]
eheader += [0].pack('N')[2..3] # Gid [u32]
eheader += [0].pack('N')[2..3] # Rdev [u32]
eheader += [0].pack('N')[2..3] # Flags [u32]

h_ci = prep_cipher
h = h_ci[:enc].update(eheader) + h_ci[:enc].final
h_len = h.length

payload = "Hello, World!"
p_ci = prep_cipher
p = p_ci[:enc].update(payload) + p_ci[:enc].final
p_len = p.length

filename = "hello-world.txt"
f_ci = prep_cipher
f = f_ci[:enc].update(filename) + f_ci[:enc].final
f_len = f.length

out = "ashring\x00"           # Magic

out += [0,h_len].pack('NN')   # Size of header

out += h_ci[:iv]              # IV of header
out += f_ci[:iv]              # IV of filename
out += p_ci[:iv]              # IV of payload

out += [0].pack('N')          # Flags

out += h
out += p

File.write((f.unpack('h' * f.length).join + '.ashring'), out)
