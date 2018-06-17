
pragma solidity ^0.4.16;

interface token {
    function transfer(address receiver, uint amount) external;
    function burn(uint256 amount) external;
    function balanceOf(address addr) external returns (uint256 amount);
}

contract Crowdsale {
    address public owner;
    address public beneficiary;
    uint public fundingGoal;
    uint public amountRaised;
    uint public tokensForSale;
    uint public tokensSold;
    uint public deadline;
    uint public tokensPerWei;
    token public tokenReward;
    mapping(address => uint256) public tokenBalanceOf;   // token balance of donors
    bool fundingGoalReached = false;
    bool crowdsaleClosed = false;
    bool public unsoldTokensBurnt = false;
    bool public unsoldTokensTransferred = false;

    event GoalReached(address recipient, uint totalAmountRaised);
    event FundTransfer(address backer, uint amount, bool isContribution);

    /**
     * Constructor function
     *
     * Setup the owner
     */
    function Crowdsale(
        address ifSuccessfulSendTo,
        uint fundingGoalInEthers,
        uint durationInMinutes,
        uint tokensForOneWei,
        address addressOfTokenUsedAsReward
    ) public {
        owner = msg.sender;
        beneficiary = ifSuccessfulSendTo;
        fundingGoal = fundingGoalInEthers * 1 ether;        // 1 ether == 1,000,000,000,000,000,000
        deadline = now + durationInMinutes * 1 minutes;
        tokensPerWei = tokensForOneWei;      // 1 wei -> 1000 tokens for now (0.001 eth == 1x10^18 tokens)
        tokenReward = token(addressOfTokenUsedAsReward);    // instantiate a contract at a given address
    }

    /**
     * Fallback function
     *
     * The function without name is the default function that is called whenever anyone sends funds to a contract
     */
    function () payable public {
        require(now < deadline);
        uint amount = msg.value;
        uint tokens = amount * tokensPerWei;
        tokenBalanceOf[msg.sender] += tokens;
        amountRaised += amount;
        tokensSold += tokens;
    }

    modifier afterDeadline() { 
        if (now >= deadline) _; 
    }

    function setTokensForSale() public {
        require(msg.sender == owner);
        require(0 == tokensForSale);    
        tokensForSale = tokenReward.balanceOf(this);
    }

    /**
     * Check if goal was reached
     *
     * Checks if the goal or time limit has been reached and ends the campaign
     */
    function checkGoalReached() afterDeadline public {
        // if (amountRaised >= fundingGoal){
        //     fundingGoalReached = true;
        //     emit GoalReached(beneficiary, amountRaised);
        // }
        crowdsaleClosed = true;
    }

    /**
     * Withdraw all tokens 
     *  - todo: support withdraw(some tokens)
     *
     * Checks to see if goal or time limit has been reached, and if so, and the funding goal was reached,
     * sends the entire amount to the beneficiary. If goal was not reached, each contributor can withdraw
     * the amount they contributed.
     */
    function withdrawTokens() afterDeadline public {
        uint tokens = tokenBalanceOf[msg.sender];
        tokenBalanceOf[msg.sender] = 0;     // fix for reentrancy bug
        if (tokens > 0) {
            tokenReward.transfer(msg.sender, tokens);
            emit FundTransfer(msg.sender, tokens, true);
        }
        // todo: add else to assign back the tokens if token transfer fails
    }

    /**
     * Withdraw the funds
     *
     * Checks to see if goal or time limit has been reached, and if so, and the funding goal was reached,
     * sends the entire amount to the beneficiary. If goal was not reached, each contributor can withdraw
     * the amount they contributed.
     */
    function withdrawFunds() afterDeadline public {
        if (beneficiary == msg.sender) {
            if (beneficiary.send(amountRaised)) {
                emit FundTransfer(beneficiary, amountRaised, false);
            } else {
                //If we fail to send the funds to beneficiary, unlock funders balance
                fundingGoalReached = false;
            }
        }
    }

    function burnUnsoldTokens() afterDeadline public {
        require(msg.sender == owner);
        require(!unsoldTokensBurnt);
        unsoldTokensBurnt = true;        
        uint256 unsoldTokens = tokensForSale - tokensSold;
        tokenReward.burn(unsoldTokens);
        // Todo: Handle return of burn function
    }    

    function transferUnsoldTokens(address toAddress) afterDeadline public {        
        require(msg.sender == owner);
        require(!unsoldTokensTransferred);
        unsoldTokensTransferred = true;                
        uint256 unsoldTokens = tokensForSale - tokensSold;
        tokenReward.transfer(toAddress, unsoldTokens);
        emit FundTransfer(toAddress, unsoldTokens, true);        
    }
}
