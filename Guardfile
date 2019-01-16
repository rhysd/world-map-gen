directories ['src']

guard :shell do
  watch /src\/.+\.rs$/ do |m|
    puts "#{Time.now}: #{m[0]}"
    success = system 'wasm-pack build'
    puts(success ? 'OK' : 'FAIL')
  end
end
