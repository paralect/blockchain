
pragma solidity ^0.4.16;

interface token {
    function transfer(address receiver, uint amount) external;
    function burn(uint256 amount) external;
    function balanceOf(address addr) external returns (uint256 amount);
}

contract Crowdsale {
    address public beneficiary;
    uint public fundingGoal;
    uint public amountRaised;
    uint public deadline;
    uint public tokensPerWei;
    token public tokenReward;
    mapping(address => uint256) public tokenBalanceOf;   // token balance of donors
    bool fundingGoalReached = false;
    bool crowdsaleClosed = false;
    bool public unsoldtokensBurnt = false;

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
        require(!crowdsaleClosed);
        uint amount = msg.value;
        tokenBalanceOf[msg.sender] += amount * tokensPerWei;
        amountRaised += amount;
    }

    modifier afterDeadline() { 
        if (now >= deadline) _; 
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
        require(!unsoldtokensBurnt);
        unsoldtokensBurnt = true;        
        uint256 amount = tokenReward.balanceOf(this);
        tokenReward.burn(amount);
        // Todo: Handle return of burn function
    }    
}
