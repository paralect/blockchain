pragma solidity ^0.4.18;

//// 'employees' accounts will manually 'withdraw' salary maximum one third of 'totalReceived'
//// money (eth) sent to (or created with) this contract
contract PaySalary {
    
    // Pay salary to these accounts equally
    address[] employees = [0x1750244Bb2A84Fc3A9d31C8c372E4D017e4E0d73, 
                           0xCedFd69893DD413BDFF0b82028b68F3766f4703a,
                           0x86dA482A00485072563447b3631DA85c383Ff1E1];

    uint totalReceived = 0;
    mapping(address => uint) withdrawnAmounts;

    // Defining constructor as 'payable' allows it to receive ethereum at the time of creation.
    function PaySalary() payable public {
        updateTotalReceived();
    }

    // Receive ethereum when somebody sends eth into this smart contract's address
    // (Fallback function, invoked ethereum is sent to the smart contract's address)
    function () payable public {
        updateTotalReceived();
    }

    function updateTotalReceived() internal {
        totalReceived += msg.value;
    }

    // Function modifiers are used to imply restrictions on the function calls.
    modifier isOurEmployee() {
        bool ourEmployee = false;

        // Check the requester (sender) is one of our employee
        for (uint i = 0; i < employees.length; i++) {
            if (employees[i] == msg.sender)
                ourEmployee = true;
                // !!! No 'break' here, so that every person pays the same gas price.
                //     Not good.
        }

        require(ourEmployee);
        _;
            //This "_;" will be replaced by the actual function body 
    }

  // This withdraw() function is called only if the condition 'isOurEmployee' is met 
  // Uses isOurEmployee modifier to check whether the address belongs to one of the employees.
  function withdraw() isOurEmployee public {
    uint amountAllocated = totalReceived / employees.length;
    uint amountWithdrawn = withdrawnAmounts[msg.sender];
    uint amount = amountAllocated - amountWithdrawn;
    withdrawnAmounts[msg.sender] = amountWithdrawn + amount;

    if (amount > 0) {
      msg.sender.transfer(amount);
    }
  }

}