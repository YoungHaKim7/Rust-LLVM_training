import lit.formats
import os

config.name = 'a01_hello_hello'
config.test_format = lit.formats.ShTest()
config.suffixes = ['.ll']

# Add tools to PATH if needed
config.llvm_tools_dir = os.path.join(os.path.dirname(__file__), '..', '..', '..', '..', '..', '..', '..', 'opt', 'homebrew', 'bin')
