require "file_utils"
require "file"

if ARGV.size < 2
  help
  exit
end

verb = ARGV.shift
profile = ARGV.shift

logic(verb, profile)

def logic(verb, profile)
  path = File.expand_path("~/.switcher-#{profile}")
  kube = File.expand_path("~/.kube")
  case verb
  when "init"
    if File.exists?(kube) && File.directory?(kube)
      puts "Moving current configuration into #{profile} profile."
      FileUtils.mv(kube, path)
    end
    logic("switch", profile)
  when "create"
    FileUtils.mkdir_p(path)

  when "switch"
    if File.exists?(kube) && !File.symlink?(kube)
      puts "Error: ~/.kube has to be a symlink or does not exist, try running"
      puts "  ./switcher init <profile>"
      puts "initialize profile from existing data."
      exit
    end

    if File.exists?(kube)
      FileUtils.rm(kube)
    end

    File.symlink(path, kube)

  when "destroy"
    FileUtils.rm_rf(path)

  else
    help
  end
end

def help
  puts "./switcher <verb> <profile>"
  puts "<verb> = init, create, switch, destroy"
  puts "<profile> = profile name"
end
