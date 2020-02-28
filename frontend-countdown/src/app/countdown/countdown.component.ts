import { Component, OnInit } from '@angular/core';
import { HttpClient} from '@angular/common/http';
import { ActivatedRoute } from "@angular/router";

import backendURL from '../backendURL';

@Component({
  selector: 'app-countdown',
  templateUrl: './countdown.component.html',
  styleUrls: ['./countdown.component.css']
})
export class CountdownComponent implements OnInit {

  theme: boolean = false;
  timeRemaining: string = "00:00:00";

  id: string = "";
  name: string = "";
  timestamp: string;

  constructor(private http: HttpClient, private route: ActivatedRoute) { }

  ngOnInit() {
    this.id = this.route.snapshot.paramMap.get("id");

    this.http.post<{
      success: boolean;
      errors: [];
      id: string;
      name: string;
      datetime: string;
    }>(backendURL + "/get-countdown",
    {
      "id": {"data": this.id, "id": "id"},
    })
    .subscribe(
      data  => {
        console.log("POST Request is successful ", data);
        if (data.success) {
          this.id = data.id;
          this.name = data.name;
          console.log(data.datetime);
          this.timestamp = data.datetime;
          var timer = setInterval(() => this.updateClock(), 1);
        } else {
          // this.errService.handleErrors(this.el, data.errors, ["email", "password"]);
        }
      },
      error  => {
        console.log("Error", error);
        // this.errService.handleErrors(this.el, [{"id": "fatal", "reason": "Unable to perform request. Please try again."}], []);
      }

    );
  }

  padZeros(data, desiredLength) {
    while (data.toString().length < desiredLength) {
      data = "0" + data.toString();
    }
    return data;
  }
  
  updateClock() {
    var currentDate = new Date();
    var targetTime = new Date(this.timestamp);
    var millisecondsDelta = targetTime.getTime() - currentDate.getTime();
    var d = new Date(Date.UTC(0, 0, 0, 0, 0, 0, millisecondsDelta));

    var days = this.padZeros(d.getUTCDay(), 2);

    var h = this.padZeros(d.getUTCHours(), 2);
    var m = this.padZeros(d.getUTCMinutes(), 2);
    var s = this.padZeros(d.getUTCSeconds(), 2);
    var ms = this.padZeros(d.getUTCMilliseconds(), 3);
    // var h = d.getUTCHours();
    // var m = d.getUTCMinutes();
    // var s = d.getUTCSeconds();
    // var ms = d.getUTCMilliseconds();
    // var time = h + ":" + m + ":" + s + ":" + ms;
    var time = h + ":" + m + ":" + s;
    if (d.getUTCDay() > 0) {
      time = days + ":" + h + ":" + m + ":" + s;
    }

    if (millisecondsDelta <= 0)
      time = "00:00:00";

    this.timeRemaining = time;
  }
  
  themeChange() {
    if (this.theme) {
      document.body.setAttribute('data-theme', 'light')
    } else {
      document.body.setAttribute('data-theme', 'dark')
    }
  }

}