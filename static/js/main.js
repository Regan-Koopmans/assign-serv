
/// Initial appointments load.

$( document ).ready(function() {
    $.ajax({url: "get-appointments", success: function(result){
        
    	$("#list").html("<br>You have the following appointments:");

    	for (appointment of JSON.parse(result).appointments) {
    		console.log(appointment);

    		$("#list").append(appointmentCard(appointment));
    	}
    }});
});


// Subsequent appointment requests.

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
	result += "<h5>" + appointment.title +"</h5>";
	result += "<h6> Description: " + appointment.desc; + "</h6>";
	result += "<h6> Date: " + appointment.date; + "</h6>";
	result += "<h6> Time: " + appointment.time; + "</h6>";
	result += "</div>";
	return result; 
}