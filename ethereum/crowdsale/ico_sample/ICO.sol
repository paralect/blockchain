
pragma solidity ^0.4.16;

interface token {
    function transfer(address receiver, uint amount) external;
    function burn(uint256 amount) external;
    function balanceOf(address addr) external returns (uint256 amount);
}

contract Crowdsale {
    address public owner;
    address public beneficiary;
    uint public amountRaised;
    uint public tokensForSale;
    uint public tokensSold;
    uint public icoDeadline;
    uint public tokensClaimableDeadline;
    uint public tokensPerWei;
    token public tokenReward;
    bool public crowdsaleClosed = false;
    bool public unsoldTokensBurnt = false;
    bool public unsoldTokensTransferred = false;

    event GoalReached(address recipient, uint totalAmountRaised);
    event FundTransfer(address backer, uint amount, bool isContribution);

    // This is a type for a single Investor
    struct Inv {
        bool whitelisted;
        uint purchasedTokens;     
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
        uint durationOfIcoInMinutes,
        uint durationTokensClaimableAfterInMinutes,
        uint tokensForOneWei,
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
        uint amount = msg.value;
        uint tokensToPurchase = amount * tokensPerWei;
        require(tokensToPurchase <= tokensForSale - tokensSold);
        investors[msg.sender].purchasedTokens += tokensToPurchase;      
        amountRaised += amount;
        tokensSold += tokensToPurchase;
    }

    function setTokensForSale() public {
        require(msg.sender == owner);
        uint totalTokens = tokenReward.balanceOf(this);
        tokensForSale = totalTokens;
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

    function checkIfIcoEnded() afterIcoDeadline public {
        require(msg.sender == owner);        
        crowdsaleClosed = true;
    }

    function burnUnsoldTokens() afterIcoDeadline public {
        require(msg.sender == owner);
        require(!unsoldTokensBurnt);
        unsoldTokensBurnt = true;
        uint256 unsoldTokens = tokensForSale - tokensSold;
        require(unsoldTokens > 0);
        tokenReward.burn(unsoldTokens);
        // Todo: Handle return of burn function
    }    

    function transferUnsoldTokens(address toAddress) afterIcoDeadline public {        
        require(msg.sender == owner);
        require(!unsoldTokensTransferred);
        unsoldTokensTransferred = true;                
        uint256 unsoldTokens = tokensForSale - tokensSold;
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
        uint tokens = investors[msg.sender].purchasedTokens;
        investors[msg.sender].purchasedTokens = 0;
        investors[msg.sender].tokensClaimed = true;
        tokenReward.transfer(msg.sender, tokens);     
        emit FundTransfer(msg.sender, tokens, true);
    }
}
