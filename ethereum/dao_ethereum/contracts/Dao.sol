// SPDX-License-Identifier: MIT
pragma solidity 0.8.18;
/// @title Simple DAO smart contract.

import "truffle/console.sol";
// or
// import "hardhat/console.sol";

contract Dao {

    enum Vote { NO, YES }

    // Create a mapping of proposal ID => Proposal
    mapping(uint256 => Proposal) public proposals;

    // Number of proposals that have been created
    uint256 public num_proposals;
    
    // todo
    // mapping(address => bool) []  // array index is proposal ID
    // or better use C++ std::set e.g. [proposal ID, hash_set of addresses)], 
    // we do not really need bools
    // proposal ID => (voter_addr => voted)
    mapping(uint256 => mapping(address => bool)) public voters;

    // todo: move to Proposal.sol
    // Create a struct named Proposal containing all relevant information
    struct Proposal {
        string name;
        uint256 yes_votes;
        uint256 no_votes;                
        uint256 deadline; // the UNIX timestamp until which this proposal 
                          // is open to voting 
    }

    function create_proposal(string memory name) public {
        Proposal storage proposal = proposals[num_proposals];
        proposal.name = name;
        proposal.deadline = block.timestamp + 90 days;
        num_proposals++;
    }

    // hint: https://www.unixtimestamp.com/
    function change_proposal_deadline(uint256 prop_id, uint256 unix_timestamp) public {
        Proposal storage proposal = proposals[prop_id];
        require(unix_timestamp > block.timestamp, "DEADLINE_CANNOT_BE_IN_THE_PAST");
        proposal.deadline = unix_timestamp;
    }

    function change_proposal_name(uint256 prop_id, string memory name) public {
        require(bytes(name).length != 0, "EMPTY_PROPOSAL_NAME");
        Proposal storage proposal = proposals[prop_id];
        proposal.name = name;
    }

    function vote_on_proposal(uint256 prop_id, Vote vote) public {
        require(voters[prop_id][msg.sender] == false, "ALREADY_VOTED");
        require(
            proposals[prop_id].deadline > block.timestamp,
            "DEADLINE_EXCEEDED"
        );
        Proposal storage proposal = proposals[prop_id];
        voters[prop_id][msg.sender] = true;
        console.log("%s voted for prop. id %d", msg.sender, prop_id);

        if (vote == Vote.YES) {
            proposal.yes_votes += 1;
        } else {
            proposal.no_votes += 1;
        }
    }

    // --------- Getters

    function proposal_yes_votes(uint256 index) public view returns(uint256) {
		return proposals[index].yes_votes;
	}

    function proposal_name(uint256 index) public view returns(string memory name) {
		return proposals[index].name;
	}

}