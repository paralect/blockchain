pragma solidity 0.4.21;

/**
* @title SafeMath by OpenZeppelin
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
        if (a == 0) {
            return 0;
        }

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

interface token {
    function transfer(address receiver, uint amount) external;
    function burn(uint256 amount) external;
    function balanceOf(address addr) external returns (uint256 amount);
}

contract Crowdsale {
    address public owner;
    address public beneficiary;
    uint256 public amountRaised;
    uint256 public tokensForSale;
    uint256 public tokensSold;
    uint256 public icoDeadline;
    uint256 public tokensClaimableDeadline;
    uint256 public tokensPerWei;
    token public tokenReward;
    bool public unsoldTokensBurnt = false;
    bool public unsoldTokensTransferred = false;

    event GoalReached(address recipient, uint totalAmountRaised);
    event FundTransfer(address backer, uint amount, bool isContribution);

    // This is a type for a single Investor
    struct Inv {
        bool whitelisted;
        uint256 purchasedTokens;     
        bool tokensClaimed;      
    }

    mapping(address => Inv) public investors;   

    /**
     * Constructor function
     *
     * Setup the owner
     */
    function Crowdsale(
        address ifSuccessfulSendTo,
        uint256 durationOfIcoInMinutes,
        uint256 durationTokensClaimableAfterInMinutes,
        uint256 tokensForOneWei,
        address addressOfTokenUsedAsReward
    ) public {
        owner = msg.sender;
        beneficiary = ifSuccessfulSendTo;
        icoDeadline = now + durationOfIcoInMinutes * 1 minutes;
        tokensClaimableDeadline = now + durationTokensClaimableAfterInMinutes * 1 minutes;
        tokensPerWei = tokensForOneWei;      // 1 wei -> 1000 tokens for now (0.001 eth == 1x10^18 tokens)
        tokenReward = token(addressOfTokenUsedAsReward);    // instantiate a contract at a given address
    }

    /**
     * Fallback function
     *
     * The function without name is the default function that is called whenever anyone sends funds to a contract
     */
    function () payable public {
        require(now < icoDeadline);
        require(investors[msg.sender].whitelisted);             
        require(msg.value >= 0.001 ether);   
        uint256 amount = msg.value;
        uint256 tokensToPurchase = SafeMath.mul(amount, tokensPerWei);
        require(tokensToPurchase <= SafeMath.sub(tokensForSale, tokensSold));
        investors[msg.sender].purchasedTokens = SafeMath.add(investors[msg.sender].purchasedTokens, tokensToPurchase);      
        amountRaised = SafeMath.add(amountRaised, amount);
        tokensSold = SafeMath.add(tokensSold, tokensToPurchase);
    }

    function setTokensForSale() public {
        require(msg.sender == owner);
        tokensForSale = tokenReward.balanceOf(this);    // tokens owned by this contract
    }
    
    function addToWhitelist(address[] addresses) public {
        require(msg.sender == owner);        
        for (uint i = 0; i < addresses.length; i++) {
            investors[addresses[i]].whitelisted = true;   
        }
    }

    function removeFromWhitelist(address[] addresses) public {
        require(msg.sender == owner);        
        for (uint i = 0; i < addresses.length; i++) {
            investors[addresses[i]].whitelisted = false;   
        }
    }    

    modifier afterIcoDeadline() { if (now >= icoDeadline) _; }

    modifier afterTokensClaimableDeadline() { if (now >= tokensClaimableDeadline) _; }

    // ----------- After ICO Deadline ------------

    /**
     * Withdraw the funds
     *
     * Checks to see if goal or time limit has been reached, and if so, and the funding goal was reached,
     * sends the entire amount to the beneficiary. If goal was not reached, each contributor can withdraw
     * the amount they contributed.
     */
    function withdrawFunds() afterIcoDeadline public {
        require(beneficiary == msg.sender);
        beneficiary.transfer(amountRaised);
        emit FundTransfer(beneficiary, amountRaised, false);        
    }

    function burnUnsoldTokens() afterIcoDeadline public {
        require(msg.sender == owner);
        require(!unsoldTokensBurnt);
        unsoldTokensBurnt = true;
        uint256 unsoldTokens = SafeMath.sub(tokensForSale, tokensSold);
        require(unsoldTokens > 0);
        tokenReward.burn(unsoldTokens);
        // Todo: Handle return of burn function
    }    

    function transferUnsoldTokens(address toAddress) afterIcoDeadline public {        
        require(msg.sender == owner);
        require(!unsoldTokensTransferred);
        unsoldTokensTransferred = true;                
        uint256 unsoldTokens = SafeMath.sub(tokensForSale, tokensSold);
        require(unsoldTokens > 0);        
        tokenReward.transfer(toAddress, unsoldTokens);
        emit FundTransfer(toAddress, unsoldTokens, true);        
    }

    // ----------- After Tokens Claimable Deadline ------------

    /**
     * Withdraw all tokens 
     *  - todo: support withdraw(some tokens)
     *
     * Checks to see if goal or time limit has been reached, and if so, and the funding goal was reached,
     * sends the entire amount to the beneficiary. If goal was not reached, each contributor can withdraw
     * the amount they contributed.
     */
    function withdrawTokens() afterTokensClaimableDeadline public {
        require(investors[msg.sender].whitelisted);                
        require(investors[msg.sender].purchasedTokens > 0);   
        require(!investors[msg.sender].tokensClaimed);        
        uint256 tokens = investors[msg.sender].purchasedTokens;
        investors[msg.sender].purchasedTokens = 0;
        investors[msg.sender].tokensClaimed = true;
        tokenReward.transfer(msg.sender, tokens);     
        emit FundTransfer(msg.sender, tokens, true);
    }
}
