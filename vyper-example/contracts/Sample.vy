stored_data: uint256
map_data: map(uint256, uint256)

@public
def set(new_value : uint256):
    self.stored_data = new_value

@public
def mapSet(key : uint256, value : uint256):
    self.map_data[key] = value

@public
@constant
def get() -> uint256:
    return self.stored_data

@public
@constant
def mapGet(key : uint256) -> uint256:
    return self.map_data[key]

