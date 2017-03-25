$("#list-appointments").click(function(){
    $.ajax({url: "get-appointments", success: function(result){
        
    	$("#list").html("<br>You have the following appointments:");

    	for (appointment of JSON.parse(result).appointments) {
    		console.log(appointment);

    		$("#list").append(appointmentCard(appointment));
    	}
    }});
});



function appointmentCard(appointment) {
	result = "<div  class='card-panel container'>";
	result += "<h4>" + appointment.title +"</h4></div>";
	return result; 
}