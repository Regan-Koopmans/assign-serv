
/// Initial appointments load.

$( document ).ready(function() {
    $.ajax({url: "get-appointments", success: function(result){
        if (result) {
            $("#list").html("<br>You have the following appointments:</br>");
                for (appointment of JSON.parse(JSON.stringify(result)).appointments) {
                $("#list").append(appointmentCard(appointment));
            }
        }
    	
    }});
    $("#edit-details").hide();
});

// Subsequent appointment requests.

$("#list-appointments").click(function(){
    $.ajax({url: "get-appointments", success: function(result){
    	$("#list").html("<br>You have the following appointments:</br>");
    	for (appointment of JSON.parse(JSON.stringify(result)).appointments) {
    		console.log(appointment);
    		$("#list").append(appointmentCard(appointment));
    	}
    }});
});

// markup a card for an appointment

function appointmentCard(appointment) {
	result = "<div  class='card-panel container'>";
	result += "<h5>" + appointment.title +"</h5>";
	result += "<h6> " + appointment.desc; + "</h6>";
	result += "<h6> <b>Date:</b> " + appointment.date; + "</h6>";
	result += "<h6> <b>Time:</b> " + appointment.time; + "</h6>";
	result += "</div>";
	return result; 
}



function getDetails() {
    $.ajax({url: "get-appointment", success: function(result){
        if (result) {
            $("#list").html("<br>You have the following appointments:");
                for (appointment of JSON.parse(JSON.stringify(result)).appointments) {
                    $("#list").append(appointmentCard(appointment));
                }
        }
    }});
    if (false) {
        $("#edit-details").show();
    }
    else {
        alert("Could not find appoinment!");
    }
}