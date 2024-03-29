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

interface EthToUsd {
    function ethToUsd() external returns (uint256 ethPriceInUsd);
}

contract Crowdsale {
    address public owner;                       // Address of the contract owner
    address public fundRaiser;                  // Address which can withraw funds raised
    uint256 public amountRaisedInWei;           // Total amount of ether raised in wei
    uint256 public tokensSold;                  // Total number of tokens sold
    uint256 public tokensClaimed;               // Total Number of tokens claimed by participants
    uint256 public icoDeadline;                 // Duration this ICO will end
    uint256 public tokensClaimableAfter;        // Duration after tokens will be claimable
    uint256 public tokensPerWei;                // How many token a buyer gets per wei 
    uint256 public ethPrice;                    // ETH to USD price from Gdax or Coinbase using oraclize
    Token public tokenReward;                   // Token being distributed 
    EthToUsd public ethToUsdContract;           // Oraclize service to get Eth to Usd rate

    bool public x1;
    bool public x2;
    bool public x3;
    uint256 public x4;
    uint256 public x5;

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
        address fundRaiserAccount,
        uint256 durationOfIcoInDays,
        uint256 durationTokensClaimableAfterInDays,
        address addressOfToken,
        address addressOfEthToUsdContract
    ) 
        public
    {
        owner = msg.sender;
        fundRaiser = fundRaiserAccount;
        icoDeadline = now + durationOfIcoInDays * 1 minutes;
        tokensClaimableAfter = now + durationTokensClaimableAfterInDays * 1 minutes;
        tokenReward = Token(addressOfToken);        
        ethToUsdContract = EthToUsd(addressOfEthToUsdContract);
        ethPrice = 200;                 // Set initial price as 1 ETH == 200$
        tokensPerWei = ethPrice * 11;   // %10 discount for Pre Sale
    }

    /**
     * Fallback function: Buy token
     * The function without name is the default function that is called whenever anyone sends funds to a contract.
     * Reserves a number tokens per participant by multiplying tokensPerWei and sent ether in wei.
     * This function is able to buy token when the following four cases are all met:
     *      - Before ICO deadline
     *      - Payer address is whitelisted in this contract
     *      - Sent ether is equal or bigger than minimum transaction (0.05 ether) 
     *      - There are enough tokens to sell in this contract (tokens balance of contract minus tokensSold)
     */
    function() payable public {
        // require(now < icoDeadline);
        // require(participants[msg.sender].whitelisted);
        // require(msg.value >= 0.05 ether); 
        x1 = (now < icoDeadline);
        x2 = (participants[msg.sender].whitelisted);
        x3 = (msg.value >= 0.01 ether); 
        uint256 tokensToBuy = SafeMath.mul(msg.value, tokensPerWei);
        x4 = tokensToBuy;       
        // require(tokensToBuy <= SafeMath.sub(tokenReward.balanceOf(this), tokensSold)); 
        x5 = SafeMath.sub(tokenReward.balanceOf(this), tokensSold);
        participants[msg.sender].tokens = SafeMath.add(participants[msg.sender].tokens, tokensToBuy);    
        amountRaisedInWei = SafeMath.add(amountRaisedInWei, msg.value);
        tokensSold = SafeMath.add(tokensSold, tokensToBuy);
    }

    /**
    * Update token price by getting the latest eth to usd price from Gdax or Coinbase 
    * (using ethToUsdContract which uses oraclize calls).
    * How tokensPerWei is calculated: 
    *   - Based on token price is 0.10$ and %10 discount for Pre Sale round
    *   - tokensPerWei = (ethToUsdPrice * 10) * (110/100);
    */ 
    function updateTokenPrice() onlyOwner public {
        ethPrice = ethToUsdContract.ethToUsd();
        tokensPerWei = ethPrice * 11;
    }

    /**
    * Add single address into the whitelist. 
    * Note: Use this function for a single address to save transaction fee
    */ 
    function addToWhitelist(address addr) onlyOwner public {
        participants[addr].whitelisted = true;   
    }

    /**
    * Remove single address from the whitelist. 
    * Note: Use this function for a single address to save transaction fee
    */ 
    function removeFromWhitelist(address addr) onlyOwner public {
        participants[addr].whitelisted = false;   
    }

    /**
    * Add multiple addresses into the whitelist. 
    * Note: Use this function for more than one address to save transaction fee
    */ 
    function addAddressesToWhitelist(address[] addresses) onlyOwner public {
        for (uint i = 0; i < addresses.length; i++) {
            participants[addresses[i]].whitelisted = true;   
        }
    }

    /**
    * Remove multiple addresses from the whitelist
    * Note: Use this function for more than one address to save transaction fee
    */ 
    function removeAddressesFromWhitelist(address[] addresses) onlyOwner public {
        for (uint i = 0; i < addresses.length; i++) {
            participants[addresses[i]].whitelisted = false;   
        }
    }

    // ----------- After ICO Deadline ------------

    /**
    * Fundraiser address claims the raised funds after ICO deadline
    */ 
    function withdrawFunds() afterIcoDeadline public {
        require(fundRaiser == msg.sender);
        fundRaiser.transfer(address(this).balance);
        emit FundTransfer(fundRaiser, address(this).balance);        
    }

    /**
    * Transfer unsold tokens after ICO deadline
    * Note: This function is designed to transfer unsold Pre-ICO tokens into Final-ICO contract.
    */ 
    function transferUnsoldTokens(address toAddress) onlyOwner afterIcoDeadline public {
        uint256 tokensUnclaimed = SafeMath.sub(tokensSold, tokensClaimed);
        uint256 unsoldTokens = SafeMath.sub(tokenReward.balanceOf(this), tokensUnclaimed);
        tokenReward.transfer(toAddress, unsoldTokens);
    }

    // ----------- After Tokens Claimable Duration ------------

    /**
    * Each participant will be able to claim his tokens after duration tokensClaimableAfter
    */ 
    function withdrawTokens() afterTokensClaimableDeadline public {
        require(participants[msg.sender].whitelisted);                
        require(!participants[msg.sender].tokensClaimed);        
        participants[msg.sender].tokensClaimed = true;
        uint256 tokens = participants[msg.sender].tokens;
        tokenReward.transfer(msg.sender, tokens); 
        tokensClaimed = SafeMath.add(tokensClaimed, tokens);
    }
}