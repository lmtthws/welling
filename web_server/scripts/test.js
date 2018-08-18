window.addEventListener('load', function() {

	var newP = document.createElement("p");
	var text = document.createTextNode("Your JS loaded");

	newP.appendChild(text);
	document.body.appendChild(newP);


	var hiButton = document.getElementById('test');
	hiButton.addEventListener('click', function() {
		var callback = function handlerServerResponse() {
			if (this.readyState === 4 && this.status === 200)
			{
				var newP = document.createElement("p");
				var response = JSON.parse(this.response);
				var text = document.createTextNode(response.test);
				newP.appendChild(text);

				document.body.appendChild(newP);				
			}

		}

		var buttonReq = new XMLHttpRequest();
		buttonReq.onreadystatechange = callback;
		buttonReq.open("POST","test/post", true);

		buttonReq.send("post_body");
	})
});
