require_relative "../lib/servicepoint_binding_uniffi"

include ServicepointBindingUniffi

connection = Connection.new("172.23.42.29:2342")

pixels = Bitmap.new_max_sized
x_offset = 0
loop do

  pixels.fill(false)

  (0..((pixels.height) -1)).each do |y|
    pixels.set((y + x_offset) % pixels.width, y, true);
  end

  command = Command.bitmap_linear_win(0, 0, pixels, CompressionCode::UNCOMPRESSED)

  connection.send(command)
  sleep 0.0005

  x_offset += 1
end


