pragma solidity ^0.4.18;

//// Pay salary to 'employees' equally automatically when contract receives money(ether).
contract PayEqualSalaryAuto {
    
    address[] employees = [0x1750244Bb2A84Fc3A9d31C8c372E4D017e4E0d73, 
                           0xCedFd69893DD413BDFF0b82028b68F3766f4703a,
                           0x86dA482A00485072563447b3631DA85c383Ff1E1];

    //mapping(address => uint) salaries;

    // Defining constructor as 'payable' allows it to receive ethereum at the time of creation.
    function PayEqualSalaryAuto() payable public {
        paySalary();
    }

    // Receive ethereum when somebody sends eth into this smart contract's address
    // (Fallback function, invoked ethereum is sent to the smart contract's address)
    function () payable public {
        paySalary();
    }

    function paySalary() internal {
        for (uint i = 0; i < employees.length; i++) {
          employees[i].transfer(msg.value/employees.length);    
        }
    }

}
