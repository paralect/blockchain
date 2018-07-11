pragma solidity 0.4.21;

/**
* @title SafeMath by OpenZeppelin (commit: 5daaf60)
* @dev Math operations with safety checks that throw on error
*/
library SafeMath {
    /**
    * @dev Multiplies two numbers, throws on overflow.
    */
    function mul(uint256 a, uint256 b) internal pure returns (uint256 c) {
        // Gas optimization: this is cheaper than asserting 'a' not being zero, but the
        // benefit is lost if 'b' is also tested.
        // See: https://github.com/OpenZeppelin/openzeppelin-solidity/pull/522
        if (a == 0) 
            return 0;

        c = a * b;
        assert(c / a == b);
        return c;
    }

    /**
    * @dev Integer division of two numbers, truncating the quotient.
    */
    function div(uint256 a, uint256 b) internal pure returns (uint256) {
        // assert(b > 0); // Solidity automatically throws when dividing by 0
        // uint256 c = a / b;
        // assert(a == b * c + a % b); // There is no case in which this doesn't hold
        return a / b;
    }

    /**
    * @dev Subtracts two numbers, throws on overflow (i.e. if subtrahend is greater than minuend).
    */
    function sub(uint256 a, uint256 b) internal pure returns (uint256) {
        assert(b <= a);
        return a - b;
    }

    /**
    * @dev Adds two numbers, throws on overflow.
    */
    function add(uint256 a, uint256 b) internal pure returns (uint256 c) {
        c = a + b;
        assert(c >= a);
        return c;
    }
}

interface Token {
    function transfer(address to, uint256 value) external returns (bool success);
    function burn(uint256 amount) external;
    function balanceOf(address owner) external returns (uint256 balance);
}

contract Crowdsale {
    address public owner;                       // Address of the ICO owner
    address public beneficiary;                 // Address where funds are collected
    uint256 public amountRaised;                // Total amount of ether raised in wei
    uint256 public tokensSold;                  // Total number of tokens sold
    uint256 public tokensClaimed;               // Total Number of tokens claimed by participants
    uint256 public icoDeadline;                 // Duration this ICO will end
    uint256 public tokensClaimableAfter;        // Duration afer tokens will be claimable
    uint256 public tokensPerWei;                // How many token units a buyer gets per wei 
    Token public tokenReward;                   // Token contract being distributed 
    // Map of crowdsale participants, address as key and Participant structure as value
    mapping(address => Participant) public participants;    

    // This is a type for a single Participant
    struct Participant {
        bool whitelisted;
        uint256 tokens;
        bool tokensClaimed;
    }

    event FundTransfer(address to, uint amount);

    modifier afterIcoDeadline() { if (now >= icoDeadline) _; }
    modifier afterTokensClaimableDeadline() { if (now >= tokensClaimableAfter) _; }
    modifier onlyOwner() { require(msg.sender == owner); _; }

    /**
     * Constructor function
     */
    function Crowdsale(
        address fundRaiser,
        uint256 durationOfIcoInMinutes,
        uint256 durationTokensClaimableAfterInMinutes,
        uint256 tokensForOneWei,
        address addressOfTokenUsedAsReward
    ) 
        public 
    {
        owner = msg.sender;
        beneficiary = fundRaiser;
        icoDeadline = now + durationOfIcoInMinutes * 1 minutes;
        tokensClaimableAfter = now + durationTokensClaimableAfterInMinutes * 1 minutes;
        tokensPerWei = tokensForOneWei;
        tokenReward = Token(addressOfTokenUsedAsReward);
    }

    /**
     * Fallback function: Buys token
     *
     * The function without name is the default function that is called whenever anyone sends funds to a contract
     */
    function() payable public {
        require(now < icoDeadline);
        require(participants[msg.sender].whitelisted);             
        require(msg.value >= 0.001 ether); 
        uint256 tokensToBuy = SafeMath.mul(msg.value, tokensPerWei);
        require(tokensToBuy <= SafeMath.sub(tokenReward.balanceOf(this), tokensSold));
        participants[msg.sender].tokens = SafeMath.add(participants[msg.sender].tokens, tokensToBuy);      
        amountRaised = SafeMath.add(amountRaised, msg.value);
        tokensSold = SafeMath.add(tokensSold, tokensToBuy);
    }
    
    function addToWhitelist(address[] addresses) onlyOwner public {
        for (uint i = 0; i < addresses.length; i++) {
            participants[addresses[i]].whitelisted = true;   
        }
    }

    function removeFromWhitelist(address[] addresses) onlyOwner public {
        for (uint i = 0; i < addresses.length; i++) {
            participants[addresses[i]].whitelisted = false;   
        }
    }

    // ----------- After ICO Deadline ------------

    function withdrawFunds() afterIcoDeadline public {
        require(beneficiary == msg.sender);
        beneficiary.transfer(address(this).balance);
        emit FundTransfer(beneficiary, address(this).balance);        
    }

    function burnUnsoldTokens()  onlyOwner afterIcoDeadline public {  
        uint256 tokensUnclaimed = SafeMath.sub(tokensSold, tokensClaimed);
        uint256 unsoldTokens = SafeMath.sub(tokenReward.balanceOf(this), tokensUnclaimed);
        tokenReward.burn(unsoldTokens);
    }    

    function transferUnsoldTokens(address toAddress) onlyOwner afterIcoDeadline public {
        uint256 tokensUnclaimed = SafeMath.sub(tokensSold, tokensClaimed);
        uint256 unsoldTokens = SafeMath.sub(tokenReward.balanceOf(this), tokensUnclaimed);
        tokenReward.transfer(toAddress, unsoldTokens);
    }

    // ----------- After Tokens Claimable Duration ------------

    function withdrawTokens() afterTokensClaimableDeadline public {
        require(participants[msg.sender].whitelisted);                
        require(!participants[msg.sender].tokensClaimed);        
        participants[msg.sender].tokensClaimed = true;
        uint256 tokens = participants[msg.sender].tokens;
        tokenReward.transfer(msg.sender, tokens); 
        tokensClaimed = SafeMath.add(tokensClaimed, tokens);
    }
}
