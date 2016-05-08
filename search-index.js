var searchIndex = {};
searchIndex["hdd_standby"] = {"doc":"Library to check the power state of a hdd","items":[[4,"PowerState","hdd_standby","The power state of an ata device",null,null],[13,"Standby","","The hdd is in the standby state (PM2, usually spun down)",0,null],[13,"Idle","","The hdd is in the idle state (PM1)",0,null],[13,"Active","","The hdd is in the active or idle state (PM0 or PM1)",0,null],[13,"Unknown","","The state of the hdd is unknown (invalid ATA response)",0,null],[4,"Error","","The error type for this crate",null,null],[13,"NoAccess","","The device file could not be opened (nonexistent or insufficient rights)",1,null],[13,"InvalidDeviceFile","","The given file is no special device file",1,null],[5,"get_power_state","","Query the power status of the given device",null,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[11,"fmt","","",0,{"inputs":[{"name":"powerstate"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",1,{"inputs":[{"name":"error"},{"name":"formatter"}],"output":{"name":"result"}}]],"paths":[[4,"PowerState"],[4,"Error"]]};
initSearch(searchIndex);
