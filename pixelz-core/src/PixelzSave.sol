// PixelzSave.sol
pragma solidity ^0.8.20;

import "./GameStateStorageEVM.sol";

contract PixelzSave {
    GameStateStorageEVM public immutable storageContract;

    constructor(address _storage) {
        storageContract = GameStateStorageEVM(_storage);
    }

    function savePixelz(bytes32 ipfsHash) external {
        // assetHash stored as hex string
        storageContract.saveGameState(_toHexString(ipfsHash));
    }

    function _toHexString(bytes32 data) internal pure returns (string memory) {
        bytes16 hexAlphabet = "0123456789abcdef";
        bytes memory str = new bytes(64);
        for (uint256 i = 0; i < 32; i++) {
            str[2*i]     = hexAlphabet[uint8(data[i] >> 4)];
            str[2*i + 1] = hexAlphabet[uint8(data[i] & 0x0f)];
        }
        return string(str);
    }
}
