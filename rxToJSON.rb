require 'json'


Dir.glob('rgss-stubs/lib/**/*.rb').each do |file|
  begin
    require_relative file
  rescue => e
    File.open('Logs/errors.log', 'a') do |f|
      f.puts("Error loading file #{file}: #{e.message}")
      f.puts(e.backtrace.join("\n"))
      f.puts
    end
  end
end


Dir.glob('pokemon_essentials/pokemon-essentials/Data/Scripts/**/*.rb').each do |file|
  begin
    require_relative file
  rescue => e
    File.open('Logs/errors.log', 'a') do |f|
      f.puts("Error loading file #{file}: #{e.message}")
      f.puts(e.backtrace.join("\n"))
      f.puts
    end
  end
end

def convert_marshal_to_json(input_filename, output_filename)
  # Load the data from the Marshal file
  data = Marshal.load(File.binread(input_filename))

  # Convert the data to JSON and write it to the output file
  File.write(output_filename, JSON.pretty_generate(data))
end

begin
  convert_marshal_to_json(ARGV[0], ARGV[1])
rescue => e
  File.open('Logs/errors.log', 'a') do |f|
    f.puts("Error converting file #{ARGV[0]}: #{e.message}")
    f.puts(e.backtrace.join("\n"))
    f.puts
  end
end
