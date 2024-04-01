require 'rake/clean'

CURRENT_DIR = File.dirname(__FILE__)

PDXINFO_FILE = 'Source/pdxinfo'.freeze

def sdk_root
  env = ENV['PLAYDATE_SDK_PATH']
  return env unless env.nil?

  File.read(File.expand_path('~/.Playdate/config')).each_line do |x|
    ssv = x.split(/\s+/)
    return ssv[1] if ssv[0] == 'SDKRoot'
  end
  raise 'cannot found SDK'
end

def load_pdxinfo
  pdxinfo = {}
  begin
    File.read(PDXINFO_FILE).each_line do |x|
      pair = x.chomp.split(/=/, 2)
      pdxinfo[pair[0]] = pair[1].strip
    end
  rescue Errno::ENOENT => e
    $stderr.puts "ERROR: #{PDXINFO_FILE} not found"
    exit 1
  end
  pdxinfo
end

PDXINFO = load_pdxinfo
SDK_ROOT = sdk_root
PDC = "#{SDK_ROOT}/bin/pdc".freeze
PLAYDATE_SIMULATOR = "#{SDK_ROOT}/bin/Playdate Simulator.app".freeze
BUILD_DIR = 'build_dir'.freeze
PDX_FILES = FileList['*.pdx']
PDX_DEBUG = FileList['*debug.pdx']
PDX_RELEASE = FileList['*release.pdx']
LUA_FILES = FileList['Source/**/*.lua']
LUALIB_DIR = 'lualib'.freeze
LUALIB_FILES = FileList["#{LUALIB_DIR}/**/*.lua"]
BUILD_TARGETS = ['Simulator', 'Device']
BUILD_TYPES = ['Debug', 'Release']
PACKAGE_DIRS = FileList['packages/*']
PACKAGE_COMPILE_FLAGS_FILES = PACKAGE_DIRS.map { |d| "#{d}/compile_flags.txt" }
COMPILE_FLAGS_FILES = FileList['**/compile_flags.txt']
RESOURCE_SRC_DIR = 'resource'.freeze
RESOURCE_DST_DIR = 'Source/resource'.freeze
RESOURCE_SRCS = FileList['resource/**/*'].exclude do |f|
  File.directory?(f) || File.extname(f) == '.tsx'
end
RESOURCE_DSTS = RESOURCE_SRCS.map do |f|
  f
    .sub(/^#{RESOURCE_SRC_DIR}\//, "#{RESOURCE_DST_DIR}/")
    .sub(/\.mp3$/, '.wav')
    .sub(/\.tmx$/, '.til')
end

directory BUILD_DIR

CLEAN.include(BUILD_DIR)
CLEAN.include('Source/pdex.*')
CLEAN.include('target')
CLOBBER.include(PDX_FILES)
CLOBBER.include(RESOURCE_DST_DIR)
CLOBBER.include(COMPILE_FLAGS_FILES)
CLOBBER.include(PACKAGE_COMPILE_FLAGS_FILES)

def all_targets_and_types(&block)
  BUILD_TARGETS.each do |target|
    BUILD_TYPES.each do |type|
      block.call(target.downcase, type.downcase)
    end
  end
end

def define_cmake_make_task(target, type, option)
  build_dir = "#{BUILD_DIR}/#{target}/#{type.downcase}"
  directory build_dir
  desc "Generate Makefile (#{target}, #{type.downcase})"
  task type.downcase => [build_dir] do |t|
    cd t.source do
      unless File.exist?('Makefile')
        sh "cmake ../../.. -DCMAKE_BUILD_TYPE=#{type} #{option}"
      end
    end
  end
end

def define_cmake_xcode_task(target, option)
  build_dir = "#{BUILD_DIR}/#{target}/xcode"
  directory build_dir
  desc "Generate Xcode project (#{target})"
  task target.downcase => build_dir do |t|
    cd t.source do
      sh "cmake ../../.. #{option} -G Xcode"
      sh 'open .'
    end
  end
end

def define_build_task(target, type, deps = [])
  build_dir = "#{BUILD_DIR}/#{target}/#{type.downcase}"
  desc "Build (#{target}, #{type.downcase})"
  task type.downcase => ["generate:#{target}:#{type.downcase}"] + deps do
    cd build_dir do
      FileList['*.dylib', '*.elf'].each do |binfile|
        rm_f binfile
      end
      sh %Q!PLAYDATE_LIB_PATH=#{File.expand_path("#{CURRENT_DIR}/#{LUALIB_DIR}")} make all!
    end
  end
end

RESOURCE_SRCS.each do |f|
end

task :default do
  p PACKAGE_DIRS
  p RESOURCE_SRCS
  p RESOURCE_DSTS
end

# binging code for pd_api.h
task :make_binding do
  sh "bindgen --use-core -o #{CURRENT_DIR}/src/bindings.rs #{SDK_ROOT}/C_API/pd_api.h -- -DTARGET_EXTENSION=1"
end

task :build_rust_simulator do
  sh "cargo build"
end

task :build_rust_device do
  sh "cargo build --target thumbv7em-none-eabihf"
end

namespace :generate do
  namespace :simulator do
    define_cmake_make_task('simulator', 'Debug', '')
    define_cmake_make_task('simulator', 'Release', '')
  end
  namespace :device do
    define_cmake_make_task('device', 'Debug', "-DCMAKE_TOOLCHAIN_FILE=#{SDK_ROOT}/C_API/buildsupport/arm.cmake")
    define_cmake_make_task('device', 'Release', "-DCMAKE_TOOLCHAIN_FILE=#{SDK_ROOT}/C_API/buildsupport/arm.cmake")
  end
  define_cmake_xcode_task('xcode', '')
end

desc 'Build'
namespace :build do
  namespace :simulator do
    define_build_task('simulator', 'Debug', [:make_binding, :build_rust_simulator])
    define_build_task('simulator', 'Release', [:make_binding, :build_rust_simulator])
  end
  namespace :device do
    define_build_task('device', 'Debug', [:make_binding, :build_rust_device])
    define_build_task('device', 'Release', [:make_binding, :build_rust_device])
  end
end

desc 'Build all'
task :build do
  all_targets_and_types do |target, type|
    dir = "#{BUILD_DIR}/#{target}/#{type}"
    sh "rake build:#{target}:#{type}" if Dir.exist?(dir)
  end
end

task conv: RESOURCE_DSTS

namespace :run do
  desc 'Run on Simulator(Debug)'
  task :debug do
    raise "no pdx file" if PDX_DEBUG.empty?
    sh "open \"#{PLAYDATE_SIMULATOR}\" #{PDX_DEBUG[0]}"
  end
  desc 'Run on Simulator'
  task :release do
    raise "no pdx file" if PDX_RELEASE.empty?
    sh "open \"#{PLAYDATE_SIMULATOR}\" #{PDX_RELEASE[0]}"
  end
end

desc 'Test'
task test: ['build:simulator:debug'] do
  cd "#{BUILD_DIR}/simulator/debug" do
    sh 'make test'
  rescue
    $stderr.puts "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n"
    $stderr.puts "FAILED\n"
    $stderr.puts "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n"
    print("================================================================================\n")
    File.read('Testing/Temporary/LastTest.log').each_line do |l|
      $stderr.print(l) if l =~ /FAILED/
    end
    print("================================================================================\n")
    raise
  end
end

rule %r{^Source/.+\.wav$} => '%{^Source/,}X.mp3' do |t|
  mkdir_p File.dirname(t.name)
  sh "ffmpeg -i #{t.source} -acodec adpcm_ima_wav -ar 44100 #{t.name}"
end

rule %r{^Source/.+\.wav$} => '%{^Source/,}X.wav' do |t|
  mkdir_p File.dirname(t.name)
  sh "ffmpeg -i #{t.source} -acodec adpcm_ima_wav -ar 44100 -ac 1 #{t.name}"
end

rule %r{^Source/.+\.png$} => '%{^Source/,}X.png' do |t|
  mkdir_p File.dirname(t.name)
  cp t.source, t.name
end

rule %r{^Source/.+\.til$} => '%{^Source/,}X.tmx' do |t|
  mkdir_p File.dirname(t.name)
  sh "ruby Tool/tmxconv.rb #{t.source} -o #{t.name}"
end
